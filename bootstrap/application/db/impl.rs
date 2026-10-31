use super::*;

/// Implementation of `Display` for `MysqlTableName`, converting MySQL table name variants to their string representations.
impl Display for MysqlTableName {
    /// Writes the MySQL table name of the variant.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The table name variant.
    /// - `&mut std::fmt::Formatter<'_>` - The formatter.
    ///
    /// # Returns
    ///
    /// - `std::fmt::Result` - The result of the formatting operation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MysqlTableName::CicdPipeline => write!(f, "cicd_pipeline"),
            MysqlTableName::CicdRun => write!(f, "cicd_run"),
            MysqlTableName::CicdJob => write!(f, "cicd_job"),
            MysqlTableName::CicdStep => write!(f, "cicd_step"),
        }
    }
}

/// Implementation of `Display` for `PostgresqlTableName`, converting PostgreSQL table name variants to their string representations.
impl Display for PostgresqlTableName {
    /// Writes the PostgreSQL table name of the variant.
    ///
    /// # Arguments
    ///
    /// - `&Self` - The table name variant.
    /// - `&mut std::fmt::Formatter<'_>` - The formatter.
    ///
    /// # Returns
    ///
    /// - `std::fmt::Result` - The result of the formatting operation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PostgresqlTableName::ChatHistory => write!(f, "chat_history"),
            PostgresqlTableName::TrackingRecord => write!(f, "tracking_record"),
            PostgresqlTableName::Shortlink => write!(f, "shortlink"),
            PostgresqlTableName::Order => write!(f, "order"),
            PostgresqlTableName::Notification => write!(f, "notification"),
            PostgresqlTableName::BlogPost => write!(f, "blog_post"),
            PostgresqlTableName::BlogComment => write!(f, "blog_comment"),
            PostgresqlTableName::BlogLike => write!(f, "blog_like"),
            PostgresqlTableName::BlogFavorite => write!(f, "blog_favorite"),
            PostgresqlTableName::BlogImage => write!(f, "blog_image"),
        }
    }
}

/// Implementation of database schema building methods for `DbBootstrap`.
impl DbBootstrap {
    /// Builds the MySQL database schema containing all CICD-related tables.
    ///
    /// # Returns
    ///
    /// - `DatabaseSchema` - The MySQL database schema with pipeline, run, job, and step tables.
    #[instrument_trace]
    pub fn build_mysql_schema() -> DatabaseSchema {
        DatabaseSchema::default()
            .add_table(TableSchema::new(
                vec![],
                MysqlTableName::CicdPipeline.to_string(),
                mysql_cicd_pipeline_table_sql().to_string(),
            ))
            .add_table(TableSchema::new(
                vec![],
                MysqlTableName::CicdRun.to_string(),
                mysql_cicd_run_table_sql().to_string(),
            ))
            .add_table(TableSchema::new(
                vec![],
                MysqlTableName::CicdJob.to_string(),
                mysql_cicd_job_table_sql().to_string(),
            ))
            .add_table(TableSchema::new(
                vec![],
                MysqlTableName::CicdStep.to_string(),
                mysql_cicd_step_table_sql().to_string(),
            ))
    }

    /// Builds the PostgreSQL database schema containing all application tables.
    ///
    /// # Returns
    ///
    /// - `DatabaseSchema` - The PostgreSQL database schema with chat, tracking, shortlink, auth, order, notification, and blog tables.
    #[instrument_trace]
    pub fn build_postgresql_schema() -> DatabaseSchema {
        DatabaseSchema::default()
            .add_table(TableSchema::new(
                vec![],
                PostgresqlTableName::ChatHistory.to_string(),
                postgresql_chat_history_table_sql().to_string(),
            ))
            .add_index(postgresql_chat_history_index_sql().to_string())
            .add_table(TableSchema::new(
                vec![],
                PostgresqlTableName::TrackingRecord.to_string(),
                postgresql_tracking_record_table_sql().to_string(),
            ))
            .add_index(postgresql_tracking_record_index_sql().to_string())
            .add_table(TableSchema::new(
                vec![],
                PostgresqlTableName::Shortlink.to_string(),
                postgresql_shortlink_table_sql().to_string(),
            ))
            .add_index(postgresql_shortlink_index_sql().to_string())
            .add_table(TableSchema::new(
                vec![],
                POSTGRESQL_AUTH_USER_TABLE_NAME.to_string(),
                postgresql_auth_user_table_sql().to_string(),
            ))
            .add_index(postgresql_auth_index_sql().to_string())
            .add_init_data(postgresql_auth_data_sql().to_string())
            .add_table(TableSchema::new(
                vec![POSTGRESQL_AUTH_USER_TABLE_NAME.to_string()],
                POSTGRESQL_ORDER_RECORD_TABLE_NAME.to_string(),
                postgresql_order_record_table_sql().to_string(),
            ))
            .add_table(TableSchema::new(
                vec![POSTGRESQL_ORDER_RECORD_TABLE_NAME.to_string()],
                POSTGRESQL_ORDER_RECORD_IMAGE_TABLE_NAME.to_string(),
                postgresql_order_record_image_table_sql().to_string(),
            ))
            .add_index(postgresql_order_index_sql().to_string())
            .add_table(TableSchema::new(
                vec![],
                PostgresqlTableName::Notification.to_string(),
                postgresql_notification_table_sql().to_string(),
            ))
            .add_index(postgresql_notification_index_sql().to_string())
            .add_table(TableSchema::new(
                vec![],
                PostgresqlTableName::BlogPost.to_string(),
                postgresql_blog_post_table_sql().to_string(),
            ))
            .add_table(TableSchema::new(
                vec![PostgresqlTableName::BlogPost.to_string()],
                PostgresqlTableName::BlogComment.to_string(),
                postgresql_blog_comment_table_sql().to_string(),
            ))
            .add_table(TableSchema::new(
                vec![PostgresqlTableName::BlogPost.to_string()],
                PostgresqlTableName::BlogLike.to_string(),
                postgresql_blog_like_table_sql().to_string(),
            ))
            .add_table(TableSchema::new(
                vec![PostgresqlTableName::BlogPost.to_string()],
                PostgresqlTableName::BlogFavorite.to_string(),
                postgresql_blog_favorite_table_sql().to_string(),
            ))
            .add_table(TableSchema::new(
                vec![PostgresqlTableName::BlogPost.to_string()],
                PostgresqlTableName::BlogImage.to_string(),
                postgresql_blog_image_table_sql().to_string(),
            ))
            .add_index(postgresql_blog_index_sql().to_string())
    }
}

/// Implementation of `BootstrapAsyncInit` for `DbBootstrap`, establishing database connections and running auto-creation on initialization.
impl BootstrapAsyncInit for DbBootstrap {
    /// Initializes the database bootstrap by establishing connections to MySQL, PostgreSQL, and Redis instances,
    /// then running the auto-creation process for all configured databases.
    ///
    /// # Returns
    ///
    /// - `Self` - The initialized `DbBootstrap` instance.
    async fn init() -> Self {
        let mysql_schema: DatabaseSchema = Self::build_mysql_schema();
        let postgresql_schema: DatabaseSchema = Self::build_postgresql_schema();
        let _: Result<DatabaseConnection, String> =
            MySqlPlugin::get_connection(default_mysql_instance_name(), Some(mysql_schema)).await;
        let _: Result<DatabaseConnection, String> = PostgreSqlPlugin::get_connection(
            default_postgresql_instance_name(),
            Some(postgresql_schema),
        )
        .await;
        let _: Result<ArcRwLock<Connection>, String> =
            RedisPlugin::get_connection(default_redis_instance_name(), None).await;
        Self
    }
}
