use super::*;

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
