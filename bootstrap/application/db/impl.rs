use super::*;

impl std::fmt::Display for MysqlTableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MysqlTableName::CicdPipeline => write!(f, "cicd_pipeline"),
            MysqlTableName::CicdRun => write!(f, "cicd_run"),
            MysqlTableName::CicdJob => write!(f, "cicd_job"),
            MysqlTableName::CicdStep => write!(f, "cicd_step"),
        }
    }
}

impl std::fmt::Display for PostgresqlTableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostgresqlTableName::ChatHistory => write!(f, "chat_history"),
            PostgresqlTableName::TrackingRecord => write!(f, "tracking_record"),
            PostgresqlTableName::Shortlink => write!(f, "shortlink"),
            PostgresqlTableName::Order => write!(f, "order"),
        }
    }
}

impl DbBootstrap {
    #[instrument_trace]
    pub fn build_mysql_schema() -> DatabaseSchema {
        DatabaseSchema::default()
            .add_table(TableSchema::new(
                Vec::new(),
                MysqlTableName::CicdPipeline.to_string(),
                MYSQL_CICD_PIPELINE_TABLE_SQL.to_string(),
            ))
            .add_table(TableSchema::new(
                Vec::new(),
                MysqlTableName::CicdRun.to_string(),
                MYSQL_CICD_RUN_TABLE_SQL.to_string(),
            ))
            .add_table(TableSchema::new(
                Vec::new(),
                MysqlTableName::CicdJob.to_string(),
                MYSQL_CICD_JOB_TABLE_SQL.to_string(),
            ))
            .add_table(TableSchema::new(
                Vec::new(),
                MysqlTableName::CicdStep.to_string(),
                MYSQL_CICD_STEP_TABLE_SQL.to_string(),
            ))
    }

    #[instrument_trace]
    pub fn build_postgresql_schema() -> DatabaseSchema {
        DatabaseSchema::default()
            .add_table(TableSchema::new(
                Vec::new(),
                PostgresqlTableName::ChatHistory.to_string(),
                POSTGRESQL_CHAT_HISTORY_TABLE_SQL.to_string(),
            ))
            .add_index(POSTGRESQL_CHAT_HISTORY_INDEX_SQL.to_string())
            .add_table(TableSchema::new(
                Vec::new(),
                PostgresqlTableName::TrackingRecord.to_string(),
                POSTGRESQL_TRACKING_RECORD_TABLE_SQL.to_string(),
            ))
            .add_index(POSTGRESQL_TRACKING_RECORD_INDEX_SQL.to_string())
            .add_table(TableSchema::new(
                Vec::new(),
                PostgresqlTableName::Shortlink.to_string(),
                POSTGRESQL_SHORTLINK_TABLE_SQL.to_string(),
            ))
            .add_index(POSTGRESQL_SHORTLINK_INDEX_SQL.to_string())
            .add_table(TableSchema::new(
                vec!["order_user".to_string()],
                "order_record".to_string(),
                POSTGRESQL_ORDER_RECORD_TABLE_SQL.to_string(),
            ))
            .add_table(TableSchema::new(
                Vec::new(),
                "order_user".to_string(),
                POSTGRESQL_ORDER_USER_TABLE_SQL.to_string(),
            ))
            .add_index(POSTGRESQL_ORDER_INDEX_SQL.to_string())
            .add_init_data(POSTGRESQL_ORDER_DATA_SQL.to_string())
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
        Self
    }
}
