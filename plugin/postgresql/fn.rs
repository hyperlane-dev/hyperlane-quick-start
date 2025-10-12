use super::*;

pub async fn connection_postgresql_db() -> Result<DatabaseConnection, String> {
    let env: &'static EnvConfig = get_global_env_config();
    match perform_postgresql_auto_creation().await {
        Ok(result) => {
            if result.has_changes() {
                crate::database::AutoCreationLogger::log_auto_creation_complete(
                    crate::database::PluginType::PostgreSQL,
                    &result,
                )
                .await;
            }
        }
        Err(error) => {
            crate::database::AutoCreationLogger::log_auto_creation_error(
                &error,
                "Auto-creation process",
                crate::database::PluginType::PostgreSQL,
                Some(&env.postgresql_database),
            )
            .await;
            if !error.should_continue() {
                return Err(error.to_string());
            }
        }
    }
    let db_url: String = format!(
        "postgres://{}:{}@{}:{}/{}",
        env.postgresql_username,
        env.postgresql_password,
        env.postgresql_host,
        env.postgresql_port,
        env.postgresql_database
    );
    Database::connect(&db_url).await.map_err(|error: DbErr| {
        let error_msg: String = error.to_string();
        futures::executor::block_on(async {
            crate::database::AutoCreationLogger::log_connection_verification(
                crate::database::PluginType::PostgreSQL,
                &env.postgresql_database,
                false,
                Some(&error_msg),
            )
            .await;
        });
        error_msg
    })
}

pub async fn get_postgresql_connection() -> Result<DatabaseConnection, String> {
    POSTGRESQL_DB.clone()
}

pub async fn perform_postgresql_auto_creation() -> Result<AutoCreationResult, AutoCreationError> {
    let start_time: Instant = Instant::now();
    let env: &'static EnvConfig = get_global_env_config();
    let mut result: AutoCreationResult = AutoCreationResult::new();
    AutoCreationLogger::log_auto_creation_start(
        crate::database::PluginType::PostgreSQL,
        &env.postgresql_database,
    )
    .await;
    let auto_creator: PostgreSqlAutoCreation = PostgreSqlAutoCreation::new();
    match auto_creator.create_database_if_not_exists().await {
        Ok(created) => {
            result.database_created = created;
        }
        Err(error) => {
            AutoCreationLogger::log_auto_creation_error(
                &error,
                "Database creation",
                crate::database::PluginType::PostgreSQL,
                Some(&env.postgresql_database),
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
        Ok(tables) => {
            result.tables_created = tables;
        }
        Err(error) => {
            AutoCreationLogger::log_auto_creation_error(
                &error,
                "Table creation",
                crate::database::PluginType::PostgreSQL,
                Some(&env.postgresql_database),
            )
            .await;
            result.errors.push(error.to_string());
        }
    }
    if let Err(error) = auto_creator.verify_connection().await {
        AutoCreationLogger::log_auto_creation_error(
            &error,
            "Connection verification",
            crate::database::PluginType::PostgreSQL,
            Some(&env.postgresql_database),
        )
        .await;
        if !error.should_continue() {
            result.duration = start_time.elapsed();
            return Err(error);
        }
        result.errors.push(error.to_string());
    }
    result.duration = start_time.elapsed();
    AutoCreationLogger::log_auto_creation_complete(
        crate::database::PluginType::PostgreSQL,
        &result,
    )
    .await;
    Ok(result)
}
