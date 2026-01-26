use super::*;

#[instrument_trace]
pub async fn connection_mysql_db<I>(instance_name: I) -> Result<DatabaseConnection, String>
where
    I: AsRef<str>,
{
    let instance_name_str: &str = instance_name.as_ref();
    let env: &'static EnvConfig = get_global_env_config();
    let instance: &MySqlInstanceConfig = env
        .get_mysql_instance(instance_name_str)
        .ok_or_else(|| format!("MySQL instance '{instance_name_str}' not found"))?;
    match perform_mysql_auto_creation(instance).await {
        Ok(result) => {
            if result.has_changes() {
                database::AutoCreationLogger::log_auto_creation_complete(
                    database::PluginType::MySQL,
                    &result,
                )
                .await;
            }
        }
        Err(error) => {
            database::AutoCreationLogger::log_auto_creation_error(
                &error,
                "Auto-creation process",
                database::PluginType::MySQL,
                Some(&instance.database),
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
                "MySQL connection timeout after {timeout_seconds} seconds"
            ))),
        };
    connection_result.map_err(|error: DbErr| {
        let error_msg: String = error.to_string();
        let database_name: String = instance.database.clone();
        let error_msg_clone: String = error_msg.clone();
        tokio::spawn(async move {
            database::AutoCreationLogger::log_connection_verification(
                database::PluginType::MySQL,
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
pub async fn get_mysql_connection<I>(instance_name: I) -> Result<DatabaseConnection, String>
where
    I: AsRef<str>,
{
    let instance_name_str: &str = instance_name.as_ref();
    let mut connections: RwLockWriteGuard<'_, HashMap<String, Result<DatabaseConnection, String>>> =
        MYSQL_CONNECTIONS.write().await;
    if let Some(connection_result) = connections.get(instance_name_str) {
        match connection_result {
            Ok(conn) => return Ok(conn.clone()),
            Err(_) => {
                connections.remove(instance_name_str);
            }
        }
    }
    drop(connections);
    let new_connection: Result<DatabaseConnection, String> =
        connection_mysql_db(instance_name_str).await;
    let mut connections: RwLockWriteGuard<'_, HashMap<String, Result<DatabaseConnection, String>>> =
        MYSQL_CONNECTIONS.write().await;
    match &new_connection {
        Ok(conn) => {
            connections.insert(instance_name_str.to_string(), Ok(conn.clone()));
        }
        Err(error) => {
            connections.insert(instance_name_str.to_string(), Err(error.clone()));
        }
    }
    new_connection
}

#[instrument_trace]
pub async fn perform_mysql_auto_creation(
    instance: &MySqlInstanceConfig,
) -> Result<AutoCreationResult, AutoCreationError> {
    let start_time: Instant = Instant::now();
    let mut result: AutoCreationResult = AutoCreationResult::default();
    AutoCreationLogger::log_auto_creation_start(database::PluginType::MySQL, &instance.database)
        .await;
    let auto_creator: MySqlAutoCreation = MySqlAutoCreation::new(instance.clone());
    match auto_creator.create_database_if_not_exists().await {
        Ok(created) => {
            result.database_created = created;
        }
        Err(error) => {
            AutoCreationLogger::log_auto_creation_error(
                &error,
                "Database creation",
                database::PluginType::MySQL,
                Some(&instance.database),
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
                database::PluginType::MySQL,
                Some(&instance.database),
            )
            .await;
            result.errors.push(error.to_string());
        }
    }
    if let Err(error) = auto_creator.verify_connection().await {
        AutoCreationLogger::log_auto_creation_error(
            &error,
            "Connection verification",
            database::PluginType::MySQL,
            Some(&instance.database),
        )
        .await;
        if !error.should_continue() {
            result.duration = start_time.elapsed();
            return Err(error);
        }
        result.errors.push(error.to_string());
    }
    result.duration = start_time.elapsed();
    AutoCreationLogger::log_auto_creation_complete(database::PluginType::MySQL, &result).await;
    Ok(result)
}
