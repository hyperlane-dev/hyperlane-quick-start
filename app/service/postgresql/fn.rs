use super::*;

async fn get_postgresql_connection() -> Result<DatabaseConnection, String> {
    let env: &'static EnvConfig = get_global_env_config();
    let db_url: String = format!(
        "postgres://{}:{}@{}:{}/{}",
        env.postgresql_username,
        env.postgresql_password,
        env.postgresql_host,
        env.postgresql_port,
        env.postgresql_database
    );
    Database::connect(&db_url).await.map_err(|e| e.to_string())
}

pub async fn create_postgresql_record(ctx: &Context) -> Result<(), String> {
    let db: DatabaseConnection = get_postgresql_connection().await?;
    let record: PostgresqlRecord = ctx
        .get_request_body_json()
        .await
        .map_err(|e| e.to_string())?;
    let active_model: ActiveModel = ActiveModel {
        key: sea_orm::ActiveValue::Set(record.key),
        value: sea_orm::ActiveValue::Set(record.value),
        id: sea_orm::ActiveValue::NotSet,
    };
    active_model.insert(&db).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn get_all_postgresql_records() -> Result<Vec<PostgresqlRecord>, String> {
    let db: DatabaseConnection = get_postgresql_connection().await?;
    let records: Vec<Model> = Entity::find().all(&db).await.map_err(|e| e.to_string())?;
    let result: Vec<PostgresqlRecord> = records
        .into_iter()
        .map(|r| PostgresqlRecord {
            key: r.key,
            value: r.value,
        })
        .collect();
    Ok(result)
}

pub async fn update_postgresql_record(ctx: &Context) -> Result<(), String> {
    let db: DatabaseConnection = get_postgresql_connection().await?;
    let record: PostgresqlRecord = ctx
        .get_request_body_json()
        .await
        .map_err(|e| e.to_string())?;
    Entity::update_many()
        .filter(Column::Key.eq(&record.key))
        .col_expr(Column::Value, Expr::value(record.value))
        .exec(&db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn delete_postgresql_record(ctx: &Context) -> Result<(), String> {
    let db: DatabaseConnection = get_postgresql_connection().await?;
    let querys: RequestQuerys = ctx.get_request_querys().await;
    let key: &String = querys.get("key").ok_or("Key parameter is required")?;
    Entity::delete_many()
        .filter(Column::Key.eq(key))
        .exec(&db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
