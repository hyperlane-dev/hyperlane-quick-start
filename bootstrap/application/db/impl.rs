use super::*;

impl DbBootstrap {
    #[instrument_trace]
    pub async fn init() {
        let _: Result<DatabaseConnection, String> =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await;
        let _: Result<DatabaseConnection, String> =
            PostgreSqlPlugin::connection_db(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await;
        let _: Result<ArcRwLock<Connection>, String> =
            RedisPlugin::connection_db(DEFAULT_REDIS_INSTANCE_NAME).await;
        match DatabasePlugin::initialize_auto_creation().await {
            Ok(_) => {
                info!("Auto-creation initialization successful");
            }
            Err(error) => {
                error!("Auto-creation initialization failed {error}");
            }
        };
    }
}
