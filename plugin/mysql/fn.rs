use super::*;

pub async fn connection_mysql_db() -> Result<DatabaseConnection, String> {
    let env: &'static EnvConfig = get_global_env_config();
    let db_url: String = format!(
        "mysql://{}:{}@{}:{}/{}",
        env.mysql_username, env.mysql_password, env.mysql_host, env.mysql_port, env.mysql_database
    );
    Database::connect(&db_url)
        .await
        .map_err(|error| error.to_string())
}

pub async fn get_mysql_connection() -> Result<DatabaseConnection, String> {
    MYSQL_DB.clone()
}
