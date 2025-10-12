use super::*;

pub async fn connection_postgresql_db() -> Result<DatabaseConnection, String> {
    let env: &'static EnvConfig = get_global_env_config();
    let db_url: String = format!(
        "postgres://{}:{}@{}:{}/{}",
        env.postgresql_username,
        env.postgresql_password,
        env.postgresql_host,
        env.postgresql_port,
        env.postgresql_database
    );
    Database::connect(&db_url)
        .await
        .map_err(|error| error.to_string())
}

pub async fn get_postgresql_connection() -> Result<DatabaseConnection, String> {
    POSTGRESQL_DB.clone()
}
