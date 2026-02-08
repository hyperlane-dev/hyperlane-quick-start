use super::*;

impl DbBootstrap {
    #[instrument_trace]
    pub fn build_mysql_schema() -> DatabaseSchema {
        DatabaseSchema::default()
            .add_table(TableSchema::new(
                Vec::new(),
                "cicd_pipeline".to_string(),
                CICD_PIPELINE_SQL.to_string(),
            ))
            .add_table(TableSchema::new(
                Vec::new(),
                "cicd_run".to_string(),
                CICD_RUN_SQL.to_string(),
            ))
            .add_table(TableSchema::new(
                Vec::new(),
                "cicd_job".to_string(),
                CICD_JOB_SQL.to_string(),
            ))
            .add_table(TableSchema::new(
                Vec::new(),
                "cicd_step".to_string(),
                CICD_STEP_SQL.to_string(),
            ))
    }

    #[instrument_trace]
    pub fn build_postgresql_schema() -> DatabaseSchema {
        let indexes: Vec<String> = POSTGRESQL_CREATE_INDEX_SQL
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty() && !s.starts_with("--"))
            .map(|s| format!("{s};"))
            .collect();
        let mut schema: DatabaseSchema = DatabaseSchema::default()
            .add_table(TableSchema::new(
                Vec::new(),
                "chat_history".to_string(),
                POSTGRESQL_CHAT_HISTORY_SQL.to_string(),
            ))
            .add_table(TableSchema::new(
                Vec::new(),
                "tracking_record".to_string(),
                POSTGRESQL_TRACKING_RECORD_SQL.to_string(),
            ))
            .add_table(TableSchema::new(
                Vec::new(),
                "shortlink".to_string(),
                POSTGRESQL_SHORTLINK_SQL.to_string(),
            ));
        for index in indexes {
            schema = schema.add_index(index);
        }
        schema
    }
}

impl BootstrapAsyncInit for DbBootstrap {
    async fn init() -> Self {
        let mysql_schema: DatabaseSchema = Self::build_mysql_schema();
        let postgresql_schema: DatabaseSchema = Self::build_postgresql_schema();
        let _: Result<DatabaseConnection, String> =
            MySqlPlugin::get_connection(DEFAULT_MYSQL_INSTANCE_NAME, Some(mysql_schema)).await;
        let _: Result<DatabaseConnection, String> = PostgreSqlPlugin::get_connection(
            DEFAULT_POSTGRESQL_INSTANCE_NAME,
            Some(postgresql_schema),
        )
        .await;
        let _: Result<ArcRwLock<Connection>, String> =
            RedisPlugin::get_connection(DEFAULT_REDIS_INSTANCE_NAME, None).await;
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
