use super::*;

#[instrument_trace]
pub fn build_mysql_schema() -> DatabaseSchema {
    DatabaseSchema::default()
        .add_table(TableSchema::new(
            Vec::new(),
            "record".to_string(),
            MYSQL_RECORD_SQL.to_string(),
        ))
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
            "record".to_string(),
            POSTGRESQL_RECORD_SQL.to_string(),
        ))
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

#[instrument_trace]
pub async fn init_db() {
    let mysql_schema: DatabaseSchema = build_mysql_schema();
    let postgresql_schema: DatabaseSchema = build_postgresql_schema();
    let _: Result<DatabaseConnection, String> =
        connection_mysql_db(DEFAULT_MYSQL_INSTANCE_NAME, Some(mysql_schema.clone())).await;
    let _: Result<DatabaseConnection, String> = connection_postgresql_db(
        DEFAULT_POSTGRESQL_INSTANCE_NAME,
        Some(postgresql_schema.clone()),
    )
    .await;
    let _: Result<Arc<Connection>, String> = connection_redis_db(DEFAULT_REDIS_INSTANCE_NAME).await;
    match initialize_auto_creation_with_schema(Some(mysql_schema), Some(postgresql_schema), None)
        .await
    {
        Ok(_) => {
            info!("Auto-creation initialization successful");
        }
        Err(error) => {
            error!("Auto-creation initialization failed{COLON_SPACE}{error}");
        }
    };
}
