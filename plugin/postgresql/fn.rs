use super::*;

#[instrument_trace]
pub async fn connection_postgresql_db<I>(
    instance_name: I,
    schema: Option<DatabaseSchema>,
) -> Result<DatabaseConnection, String>
where
    I: AsRef<str>,
{
    let instance_name_str: &str = instance_name.as_ref();
    let env: &'static EnvConfig = get_global_env_config();
    let instance: &PostgreSqlInstanceConfig = env
        .get_postgresql_instance(instance_name_str)
        .ok_or_else(|| format!("PostgreSQL instance '{instance_name_str}' not found"))?;
    match perform_postgresql_auto_creation(instance, schema.clone()).await {
        Ok(result) => {
            if result.has_changes() {
                database::AutoCreationLogger::log_auto_creation_complete(
                    database::PluginType::PostgreSQL,
                    &result,
                )
                .await;
            }
        }
        Err(error) => {
            database::AutoCreationLogger::log_auto_creation_error(
                &error,
                "Auto-creation process",
                database::PluginType::PostgreSQL,
                Some(instance.get_database().as_str()),
            )
            .await;
            if !error.should_continue() {
                return Err(error.to_string());
            }
        }
    }
    let db_url: String = instance.get_connection_url();
    let timeout_duration: Duration = get_connection_timeout_duration();
    let timeout_seconds: u64 = timeout_duration.as_secs();
    let connection_result: Result<DatabaseConnection, DbErr> =
        match timeout(timeout_duration, Database::connect(&db_url)).await {
            Ok(result) => result,
            Err(_) => Err(DbErr::Custom(format!(
                "PostgreSQL connection timeout after {timeout_seconds} seconds"
            ))),
        };
    connection_result.map_err(|error: DbErr| {
        let error_msg: String = error.to_string();
        let database_name: String = instance.get_database().clone();
        let error_msg_clone: String = error_msg.clone();
        tokio::spawn(async move {
            database::AutoCreationLogger::log_connection_verification(
                database::PluginType::PostgreSQL,
                &database_name,
                false,
                Some(&error_msg_clone),
            )
            .await;
        });
        error_msg
    })
}

#[instrument_trace]
pub async fn get_postgresql_connection<I>(
    instance_name: I,
    schema: Option<DatabaseSchema>,
) -> Result<DatabaseConnection, String>
where
    I: AsRef<str>,
{
    let instance_name_str: &str = instance_name.as_ref();
    let cooldown_duration: Duration = get_retry_cooldown_duration();
    {
        if let Some(cache) = POSTGRESQL_CONNECTIONS.read().await.get(instance_name_str) {
            match cache.try_get_result() {
                Ok(conn) => return Ok(conn.clone()),
                Err(error) => {
                    if !cache.is_cooldown_expired(cooldown_duration) {
                        return Err(error.clone());
                    }
                }
            }
        }
    }
    let mut connections: RwLockWriteGuard<
        '_,
        HashMap<String, ConnectionCache<DatabaseConnection>>,
    > = POSTGRESQL_CONNECTIONS.write().await;
    if let Some(cache) = connections.get(instance_name_str) {
        match cache.try_get_result() {
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
    let new_connection: Result<DatabaseConnection, String> =
        connection_postgresql_db(instance_name_str, schema).await;
    let mut connections: RwLockWriteGuard<
        '_,
        HashMap<String, ConnectionCache<DatabaseConnection>>,
    > = POSTGRESQL_CONNECTIONS.write().await;
    connections.insert(
        instance_name_str.to_string(),
        ConnectionCache::new(new_connection.clone()),
    );
    new_connection
}

#[instrument_trace]
pub async fn perform_postgresql_auto_creation(
    instance: &PostgreSqlInstanceConfig,
    schema: Option<DatabaseSchema>,
) -> Result<AutoCreationResult, AutoCreationError> {
    let start_time: Instant = Instant::now();
    let mut result: AutoCreationResult = AutoCreationResult::default();
    AutoCreationLogger::log_auto_creation_start(
        database::PluginType::PostgreSQL,
        instance.get_database(),
    )
    .await;
    let auto_creator: PostgreSqlAutoCreation = match schema {
        Some(s) => PostgreSqlAutoCreation::with_schema(instance.clone(), s),
        None => PostgreSqlAutoCreation::new(instance.clone()),
    };
    match auto_creator.create_database_if_not_exists().await {
        Ok(created) => {
            result.set_database_created(created);
        }
        Err(error) => {
            AutoCreationLogger::log_auto_creation_error(
                &error,
                "Database creation",
                database::PluginType::PostgreSQL,
                Some(instance.get_database().as_str()),
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
        Ok(tables) => {
            result.set_tables_created(tables);
        }
        Err(error) => {
            AutoCreationLogger::log_auto_creation_error(
                &error,
                "Table creation",
                database::PluginType::PostgreSQL,
                Some(instance.get_database().as_str()),
            )
            .await;
            result.get_mut_errors().push(error.to_string());
        }
    }
    if let Err(error) = auto_creator.verify_connection().await {
        AutoCreationLogger::log_auto_creation_error(
            &error,
            "Connection verification",
            database::PluginType::PostgreSQL,
            Some(instance.get_database().as_str()),
        )
        .await;
        if !error.should_continue() {
            result.set_duration(start_time.elapsed());
            return Err(error);
        }
        result.get_mut_errors().push(error.to_string());
    }
    result.set_duration(start_time.elapsed());
    AutoCreationLogger::log_auto_creation_complete(database::PluginType::PostgreSQL, &result).await;
    Ok(result)
}
