use super::*;

#[instrument_trace]
pub async fn init_db() {
    let _: Result<DatabaseConnection, String> =
        connection_mysql_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await;
    let _: Result<DatabaseConnection, String> =
        connection_postgresql_db(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await;
    let _: Result<ArcRwLock<Connection>, String> =
        connection_redis_db(DEFAULT_REDIS_INSTANCE_NAME).await;
    match initialize_auto_creation().await {
        Ok(_) => {
            info!("Auto-creation initialization successful");
        }
        Err(error) => {
            error!("Auto-creation initialization failed {error}");
        }
    };
}
