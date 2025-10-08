use super::*;

pub async fn connection_mysql_db() -> Pool<MySql> {
    let env: &'static EnvConfig = get_global_env_config();
    let db_url: String = format!(
        "mysql://{}:{}@{}:{}/{}",
        env.mysql_username, env.mysql_password, env.mysql_host, env.mysql_port, env.mysql_database
    );
    let pool: Pool<MySql> = MySqlPoolOptions::new()
        .max_lifetime(None)
        .test_before_acquire(false)
        .idle_timeout(None)
        .connect(&db_url)
        .await
        .unwrap();
    pool
}

pub async fn get_mysql_db() -> Pool<MySql> {
    MYSQL_DB.clone()
}
