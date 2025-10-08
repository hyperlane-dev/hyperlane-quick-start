use super::*;

pub async fn connection_postgresql_db() -> Pool<Postgres> {
    let env: &'static EnvConfig = get_global_env_config();
    let db_url: String = format!(
        "postgresql://{}:{}@{}:{}/{}",
        env.postgresql_username,
        env.postgresql_password,
        env.postgresql_host,
        env.postgresql_port,
        env.postgresql_database
    );
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_lifetime(None)
        .test_before_acquire(false)
        .idle_timeout(None)
        .connect(&db_url)
        .await
        .unwrap();
    pool
}

pub async fn get_postgresql_db() -> Pool<Postgres> {
    POSTGRESQL_DB.clone()
}
