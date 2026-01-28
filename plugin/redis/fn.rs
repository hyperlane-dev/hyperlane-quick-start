use super::*;

#[instrument_trace]
pub async fn connection_redis_db<I>(instance_name: I) -> Result<Arc<Connection>, String>
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
                Some(&instance.name),
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
    Ok(Arc::new(connection))
}

#[instrument_trace]
pub async fn get_redis_connection<I>(instance_name: I) -> Result<Arc<Connection>, String>
where
    I: AsRef<str>,
{
    let instance_name_str: &str = instance_name.as_ref();
    let cooldown_duration: Duration = get_retry_cooldown_duration();
    {
        let connections: tokio::sync::RwLockReadGuard<'_, RedisConnectionMap> =
            REDIS_CONNECTIONS.read().await;
        if let Some(cache) = connections.get(instance_name_str) {
            match &cache.result {
                Ok(conn) => return Ok(conn.clone()),
                Err(error) => {
                    if !cache.is_cooldown_expired(cooldown_duration) {
                        return Err(error.clone());
                    }
                }
            }
        }
    }
    let mut connections: RwLockWriteGuard<'_, RedisConnectionMap> = REDIS_CONNECTIONS.write().await;
    if let Some(cache) = connections.get(instance_name_str) {
        match &cache.result {
            Ok(conn) => return Ok(conn.clone()),
            Err(error) => {
                if !cache.is_cooldown_expired(cooldown_duration) {
                    return Err(error.clone());
                }
            }
        }
    }
    connections.remove(instance_name_str);
    drop(connections);
    let new_connection: Result<Arc<Connection>, String> =
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
    AutoCreationLogger::log_auto_creation_start(database::PluginType::Redis, &instance.name).await;
    let auto_creator: RedisAutoCreation = RedisAutoCreation::new(instance.clone());
    match auto_creator.create_database_if_not_exists().await {
        Ok(created) => {
            result.database_created = created;
        }
        Err(error) => {
            AutoCreationLogger::log_auto_creation_error(
                &error,
                "Database validation",
                database::PluginType::Redis,
                Some(&instance.name),
            )
            .await;
            if !error.should_continue() {
                result.duration = start_time.elapsed();
                return Err(error);
            }
            result.errors.push(error.to_string());
        }
    }
    match auto_creator.create_tables_if_not_exist().await {
        Ok(operations) => {
            result.tables_created = operations;
        }
        Err(error) => {
            AutoCreationLogger::log_auto_creation_error(
                &error,
                "Namespace setup",
                database::PluginType::Redis,
                Some(&instance.name),
            )
            .await;
            result.errors.push(error.to_string());
        }
    }
    if let Err(error) = auto_creator.verify_connection().await {
        AutoCreationLogger::log_auto_creation_error(
            &error,
            "Connection verification",
            database::PluginType::Redis,
            Some(&instance.name),
        )
        .await;
        if !error.should_continue() {
            result.duration = start_time.elapsed();
            return Err(error);
        }
        result.errors.push(error.to_string());
    }
    result.duration = start_time.elapsed();
    AutoCreationLogger::log_auto_creation_complete(database::PluginType::Redis, &result).await;
    Ok(result)
}
