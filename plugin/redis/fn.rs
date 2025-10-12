use super::*;

pub async fn connection_redis_db() -> Result<Arc<Connection>, String> {
    let env: &'static EnvConfig = get_global_env_config();
    match perform_redis_auto_creation().await {
        Ok(result) => {
            if result.has_changes() {
                crate::database::AutoCreationLogger::log_auto_creation_complete(
                    crate::database::PluginType::Redis,
                    &result,
                )
                .await;
            }
        }
        Err(error) => {
            crate::database::AutoCreationLogger::log_auto_creation_error(
                &error,
                "Auto-creation process",
                crate::database::PluginType::Redis,
                Some("default"),
            )
            .await;
            if !error.should_continue() {
                return Err(error.to_string());
            }
        }
    }
    let db_url: String = format!(
        "redis://{}:{}@{}:{}",
        env.redis_username, env.redis_password, env.redis_host, env.redis_port,
    );
    let client: Client = Client::open(db_url).map_err(|error: redis::RedisError| {
        let error_msg: String = error.to_string();
        futures::executor::block_on(async {
            crate::database::AutoCreationLogger::log_connection_verification(
                crate::database::PluginType::Redis,
                "default",
                false,
                Some(&error_msg),
            )
            .await;
        });
        error_msg
    })?;
    let connection: Connection = client
        .get_connection()
        .map_err(|error: redis::RedisError| {
            let error_msg: String = error.to_string();
            futures::executor::block_on(async {
                crate::database::AutoCreationLogger::log_connection_verification(
                    crate::database::PluginType::Redis,
                    "default",
                    false,
                    Some(&error_msg),
                )
                .await;
            });
            error_msg
        })?;
    Ok(Arc::new(connection))
}

pub async fn get_redis_connection() -> Result<Arc<Connection>, String> {
    REDIS_DB.clone()
}

pub async fn perform_redis_auto_creation() -> Result<AutoCreationResult, AutoCreationError> {
    let start_time: Instant = Instant::now();
    let mut result: AutoCreationResult = AutoCreationResult::new();
    AutoCreationLogger::log_auto_creation_start(crate::database::PluginType::Redis, "default")
        .await;
    let auto_creator: RedisAutoCreation = RedisAutoCreation::new();
    match auto_creator.create_database_if_not_exists().await {
        Ok(created) => {
            result.database_created = created;
        }
        Err(error) => {
            AutoCreationLogger::log_auto_creation_error(
                &error,
                "Database validation",
                crate::database::PluginType::Redis,
                Some("default"),
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
                crate::database::PluginType::Redis,
                Some("default"),
            )
            .await;
            result.errors.push(error.to_string());
        }
    }
    if let Err(error) = auto_creator.verify_connection().await {
        AutoCreationLogger::log_auto_creation_error(
            &error,
            "Connection verification",
            crate::database::PluginType::Redis,
            Some("default"),
        )
        .await;
        if !error.should_continue() {
            result.duration = start_time.elapsed();
            return Err(error);
        }
        result.errors.push(error.to_string());
    }
    result.duration = start_time.elapsed();
    AutoCreationLogger::log_auto_creation_complete(crate::database::PluginType::Redis, &result)
        .await;
    Ok(result)
}
