use super::*;

pub async fn connection_redis_db() -> Arc<Connection> {
    let env: &'static EnvConfig = get_global_env_config();
    let db_url: String = format!(
        "redis://{}:{}@{}:{}",
        env.redis_username, env.redis_password, env.redis_host, env.redis_port,
    );
    let client: Client = Client::open(db_url).unwrap();
    let connection: Connection = client.get_connection().unwrap();
    Arc::new(connection)
}

pub async fn get_redis_db() -> Arc<Connection> {
    REDIS_DB.clone()
}
