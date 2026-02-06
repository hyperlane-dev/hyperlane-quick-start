use super::*;

#[instrument_trace]
pub async fn connection_redis_db<I>(instance_name: I) -> Result<ArcRwLock<Connection>, String>
where
    I: AsRef<str>,
{
    let instance_name_str: &str = instance_name.as_ref();
    let env: &'static EnvConfig = get_global_env_config();
    let instance: &RedisInstanceConfig = env
        .get_redis_instance(instance_name_str)
        .ok_or_else(|| format!("Redis instance '{instance_name_str}' not found"))?;
    match perform_redis_auto_creation(instance).await {
        Ok(result) => {
            if result.has_changes() {
                database::AutoCreationLogger::log_auto_creation_complete(
                    database::PluginType::Redis,
                    &result,
                )
                .await;
            }
        }
        Err(error) => {
            database::AutoCreationLogger::log_auto_creation_error(
                &error,
                "Auto-creation process",
                database::PluginType::Redis,
                Some(instance.get_name().as_str()),
            )
            .await;
            if !error.should_continue() {
                return Err(error.to_string());
            }
        }
    }
    let db_url: String = instance.get_connection_url();
    let client: Client = Client::open(db_url).map_err(|error: redis::RedisError| {
        let error_msg: String = error.to_string();
        let instance_name_clone: String = instance_name_str.to_string();
        let error_msg_clone: String = error_msg.clone();
        tokio::spawn(async move {
            database::AutoCreationLogger::log_connection_verification(
                database::PluginType::Redis,
                &instance_name_clone,
                false,
                Some(&error_msg_clone),
            )
            .await;
        });
        error_msg
    })?;
    let timeout_duration: Duration = get_connection_timeout_duration();
    let timeout_seconds: u64 = timeout_duration.as_secs();
    let connection_task: JoinHandle<Result<Connection, RedisError>> =
        spawn_blocking(move || client.get_connection());
    let connection: Connection = match timeout(timeout_duration, connection_task).await {
        Ok(join_result) => match join_result {
            Ok(result) => result.map_err(|error: redis::RedisError| {
                let error_msg: String = error.to_string();
                let instance_name_clone: String = instance_name_str.to_string();
                let error_msg_clone: String = error_msg.clone();
                tokio::spawn(async move {
                    database::AutoCreationLogger::log_connection_verification(
                        database::PluginType::Redis,
                        &instance_name_clone,
                        false,
                        Some(&error_msg_clone),
                    )
                    .await;
                });
                error_msg
            })?,
            Err(_) => {
                let error_msg: String = "Redis connection task failed".to_string();
                let instance_name_clone: String = instance_name_str.to_string();
                let error_msg_clone: String = error_msg.clone();
                tokio::spawn(async move {
                    database::AutoCreationLogger::log_connection_verification(
                        database::PluginType::Redis,
                        &instance_name_clone,
                        false,
                        Some(&error_msg_clone),
                    )
                    .await;
                });
                return Err(error_msg);
            }
        },
        Err(_) => {
            let error_msg: String =
                format!("Redis connection timeout after {timeout_seconds} seconds");
            let instance_name_clone: String = instance_name_str.to_string();
            let error_msg_clone: String = error_msg.clone();
            tokio::spawn(async move {
                database::AutoCreationLogger::log_connection_verification(
                    database::PluginType::Redis,
                    &instance_name_clone,
                    false,
                    Some(&error_msg_clone),
                )
                .await;
            });
            return Err(error_msg);
        }
    };
    Ok(arc_rwlock(connection))
}

#[instrument_trace]
pub async fn get_redis_connection<I>(instance_name: I) -> Result<ArcRwLock<Connection>, String>
where
    I: AsRef<str>,
{
    let instance_name_str: &str = instance_name.as_ref();
    let duration: Duration = get_retry_duration();
    {
        if let Some(cache) = REDIS_CONNECTIONS.read().await.get(instance_name_str) {
            match cache.try_get_result() {
                Ok(conn) => return Ok(conn.clone()),
                Err(error) => {
                    if !cache.is_expired(duration) {
                        return Err(error.clone());
                    }
                }
            }
        }
    }
    let mut connections: RwLockWriteGuard<'_, RedisConnectionMap> = REDIS_CONNECTIONS.write().await;
    if let Some(cache) = connections.get(instance_name_str) {
        match cache.try_get_result() {
            Ok(conn) => return Ok(conn.clone()),
            Err(error) => {
                if !cache.is_expired(duration) {
                    return Err(error.clone());
                }
            }
        }
    }
    connections.remove(instance_name_str);
    drop(connections);
    let new_connection: Result<ArcRwLock<Connection>, String> =
        connection_redis_db(instance_name_str).await;
    let mut connections: RwLockWriteGuard<'_, RedisConnectionMap> = REDIS_CONNECTIONS.write().await;
    connections.insert(
        instance_name_str.to_string(),
        ConnectionCache::new(new_connection.clone()),
    );
    new_connection
}

#[instrument_trace]
pub async fn perform_redis_auto_creation(
    instance: &RedisInstanceConfig,
) -> Result<AutoCreationResult, AutoCreationError> {
    let start_time: Instant = Instant::now();
    let mut result: AutoCreationResult = AutoCreationResult::default();
    AutoCreationLogger::log_auto_creation_start(database::PluginType::Redis, instance.get_name())
        .await;
    let auto_creator: RedisAutoCreation = RedisAutoCreation::new(instance.clone());
    match auto_creator.create_database_if_not_exists().await {
        Ok(created) => {
            result.set_database_created(created);
        }
        Err(error) => {
            AutoCreationLogger::log_auto_creation_error(
                &error,
                "Database validation",
                database::PluginType::Redis,
                Some(instance.get_name().as_str()),
            )
            .await;
            if !error.should_continue() {
                result.set_duration(start_time.elapsed());
                return Err(error);
            }
            result.get_mut_errors().push(error.to_string());
        }
    }
    match auto_creator.create_tables_if_not_exist().await {
        Ok(operations) => {
            result.set_tables_created(operations);
        }
        Err(error) => {
            AutoCreationLogger::log_auto_creation_error(
                &error,
                "Namespace setup",
                database::PluginType::Redis,
                Some(instance.get_name().as_str()),
            )
            .await;
            result.get_mut_errors().push(error.to_string());
        }
    }
    if let Err(error) = auto_creator.verify_connection().await {
        AutoCreationLogger::log_auto_creation_error(
            &error,
            "Connection verification",
            database::PluginType::Redis,
            Some(instance.get_name().as_str()),
        )
        .await;
        if !error.should_continue() {
            result.set_duration(start_time.elapsed());
            return Err(error);
        }
        result.get_mut_errors().push(error.to_string());
    }
    result.set_duration(start_time.elapsed());
    AutoCreationLogger::log_auto_creation_complete(database::PluginType::Redis, &result).await;
    Ok(result)
}
