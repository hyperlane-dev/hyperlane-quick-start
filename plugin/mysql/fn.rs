use super::*;

pub async fn connection_mysql_db() -> Result<DatabaseConnection, String> {
    let env: &'static EnvConfig = get_global_env_config();
    if env.enable_auto_db_creation || env.enable_auto_table_creation {
        match perform_mysql_auto_creation().await {
            Ok(result) => {
                if result.has_changes() {
                    crate::database::AutoCreationLogger::log_auto_creation_complete(
                        crate::database::PluginType::MySQL,
                        &result,
                    )
                    .await;
                }
            }
            Err(error) => {
                crate::database::AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Auto-creation process",
                    crate::database::PluginType::MySQL,
                    Some(&env.mysql_database),
                )
                .await;
                if !error.should_continue() {
                    return Err(error.to_string());
                }
            }
        }
    }
    let db_url: String = format!(
        "mysql://{}:{}@{}:{}/{}",
        env.mysql_username, env.mysql_password, env.mysql_host, env.mysql_port, env.mysql_database
    );
    Database::connect(&db_url)
        .await
        .map_err(|error: sea_orm::DbErr| {
            let error_msg: String = error.to_string();
            futures::executor::block_on(async {
                crate::database::AutoCreationLogger::log_connection_verification(
                    crate::database::PluginType::MySQL,
                    &env.mysql_database,
                    false,
                    Some(&error_msg),
                )
                .await;
            });
            error_msg
        })
}

pub async fn get_mysql_connection() -> Result<DatabaseConnection, String> {
    MYSQL_DB.clone()
}

pub async fn perform_mysql_auto_creation() -> Result<AutoCreationResult, AutoCreationError> {
    let start_time: Instant = Instant::now();
    let env: &'static EnvConfig = get_global_env_config();
    let mut result: AutoCreationResult = AutoCreationResult::new();
    AutoCreationLogger::log_auto_creation_start(
        crate::database::PluginType::MySQL,
        &env.mysql_database,
    )
    .await;
    if !env.enable_auto_db_creation && !env.enable_auto_table_creation {
        AutoCreationLogger::log_auto_creation_disabled(
            crate::database::PluginType::MySQL,
            "Both database and table auto-creation are disabled",
        )
        .await;
        result.duration = start_time.elapsed();
        return Ok(result);
    }
    let auto_creator: MySqlAutoCreation = MySqlAutoCreation::new();
    if env.enable_auto_db_creation {
        match auto_creator.create_database_if_not_exists().await {
            Ok(created) => {
                result.database_created = created;
            }
            Err(error) => {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Database creation",
                    crate::database::PluginType::MySQL,
                    Some(&env.mysql_database),
                )
                .await;
                if !error.should_continue() {
                    result.duration = start_time.elapsed();
                    return Err(error);
                }
                result.errors.push(error.to_string());
            }
        }
    } else {
        AutoCreationLogger::log_auto_creation_disabled(
            crate::database::PluginType::MySQL,
            "Database auto-creation is disabled",
        )
        .await;
    }
    if env.enable_auto_table_creation {
        match auto_creator.create_tables_if_not_exist().await {
            Ok(tables) => {
                result.tables_created = tables;
            }
            Err(error) => {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Table creation",
                    crate::database::PluginType::MySQL,
                    Some(&env.mysql_database),
                )
                .await;
                result.errors.push(error.to_string());
            }
        }
    } else {
        AutoCreationLogger::log_auto_creation_disabled(
            crate::database::PluginType::MySQL,
            "Table auto-creation is disabled",
        )
        .await;
    }
    if let Err(error) = auto_creator.verify_connection().await {
        AutoCreationLogger::log_auto_creation_error(
            &error,
            "Connection verification",
            crate::database::PluginType::MySQL,
            Some(&env.mysql_database),
        )
        .await;
        if !error.should_continue() {
            result.duration = start_time.elapsed();
            return Err(error);
        }
        result.errors.push(error.to_string());
    }
    result.duration = start_time.elapsed();
    AutoCreationLogger::log_auto_creation_complete(crate::database::PluginType::MySQL, &result)
        .await;
    Ok(result)
}
