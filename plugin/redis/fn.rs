use super::*;

pub async fn connection_redis_db() -> Result<Arc<Connection>, String> {
    let env: &'static EnvConfig = get_global_env_config();
    let db_url: String = format!(
        "redis://{}:{}@{}:{}",
        env.redis_username, env.redis_password, env.redis_host, env.redis_port,
    );
    let client: Client = Client::open(db_url).map_err(|error| error.to_string())?;
    let connection: Connection = client.get_connection().map_err(|error| error.to_string())?;
    Ok(Arc::new(connection))
}

pub async fn get_redis_connection() -> Result<Arc<Connection>, String> {
    REDIS_DB.clone()
}
