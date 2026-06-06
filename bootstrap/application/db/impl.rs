use super::*;

/// Implementation of `BootstrapAsyncInit` for `DbBootstrap`, establishing database connections and running auto-creation on initialization.
impl BootstrapAsyncInit for DbBootstrap {
    /// Initializes the database bootstrap by establishing connections to MySQL, PostgreSQL, and Redis instances,
    /// then running the auto-creation process for all configured databases.
    ///
    /// # Returns
    ///
    /// - `Self`: The initialized `DbBootstrap` instance.
    async fn init() -> Self {
        let _: Result<DatabaseConnection, String> =
            MySqlPlugin::connection_db(DEFAULT_MYSQL_INSTANCE_NAME, None).await;
        let _: Result<DatabaseConnection, String> =
            PostgreSqlPlugin::connection_db(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await;
        let _: Result<ArcRwLock<Connection>, String> =
            RedisPlugin::connection_db(DEFAULT_REDIS_INSTANCE_NAME, None).await;
        match DatabasePlugin::initialize_auto_creation().await {
            Ok(_) => {
                info!("Auto-creation initialization successful");
            }
            Err(error) => {
                error!("Auto-creation initialization failed {error}");
            }
        };
        Self
    }
}
