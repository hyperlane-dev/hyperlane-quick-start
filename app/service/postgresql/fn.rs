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

pub async fn create_postgresql_record(ctx: Context) {
    let db: DatabaseConnection = match get_postgresql_connection().await {
        Ok(db) => db,
        Err(e) => {
            ctx.set_response_body(e.as_bytes()).await;
            return;
        }
    };
    let record: PostgresqlRecord = match ctx.get_request_body_json().await {
        Ok(r) => r,
        Err(e) => {
            ctx.set_response_body(e.to_string().as_bytes()).await;
            return;
        }
    };
    let active_model: ActiveModel = ActiveModel {
        key: sea_orm::ActiveValue::Set(record.key),
        value: sea_orm::ActiveValue::Set(record.value),
        id: sea_orm::ActiveValue::NotSet,
    };
    match active_model.insert(&db).await {
        Ok(_) => ctx.set_response_body(b"Record created successfully").await,
        Err(e) => ctx.set_response_body(e.to_string().as_bytes()).await,
    };
}

pub async fn get_all_postgresql_records(ctx: Context) {
    let db: DatabaseConnection = match get_postgresql_connection().await {
        Ok(db) => db,
        Err(e) => {
            ctx.set_response_body(e.as_bytes()).await;
            return;
        }
    };
    let records: Result<Vec<Model>, DbErr> = Entity::find().all(&db).await;
    match records {
        Ok(records) => {
            let result: Vec<PostgresqlRecord> = records
                .into_iter()
                .map(|r| PostgresqlRecord {
                    key: r.key,
                    value: r.value,
                })
                .collect();
            let json_result = serde_json::to_string(&result).unwrap_or_else(|_| "[]".to_string());
            ctx.set_response_body(json_result.as_bytes()).await;
        }
        Err(e) => {
            ctx.set_response_body(e.to_string().as_bytes()).await;
        }
    };
}

pub async fn update_postgresql_record(ctx: Context) {
    let db: DatabaseConnection = match get_postgresql_connection().await {
        Ok(db) => db,
        Err(e) => {
            ctx.set_response_body(e.as_bytes()).await;
            return;
        }
    };
    let record: PostgresqlRecord = match ctx.get_request_body_json().await {
        Ok(r) => r,
        Err(e) => {
            ctx.set_response_body(e.to_string().as_bytes()).await;
            return;
        }
    };
    let update_result: Result<UpdateResult, DbErr> = Entity::update_many()
        .filter(Column::Key.eq(&record.key))
        .col_expr(Column::Value, Expr::value(record.value))
        .exec(&db)
        .await;
    match update_result {
        Ok(_) => ctx.set_response_body(b"Record updated successfully").await,
        Err(e) => ctx.set_response_body(e.to_string().as_bytes()).await,
    };
}

pub async fn delete_postgresql_record(ctx: Context) {
    let db: DatabaseConnection = match get_postgresql_connection().await {
        Ok(db) => db,
        Err(e) => {
            ctx.set_response_body(e.as_bytes()).await;
            return;
        }
    };
    let querys: RequestQuerys = ctx.get_request_querys().await;
    let key: &String = match querys.get("key") {
        Some(k) => k,
        None => {
            ctx.set_response_body(b"Key parameter is required").await;
            return;
        }
    };
    let delete_result: Result<DeleteResult, DbErr> = Entity::delete_many()
        .filter(Column::Key.eq(key))
        .exec(&db)
        .await;
    match delete_result {
        Ok(_) => ctx.set_response_body(b"Record deleted successfully").await,
        Err(e) => ctx.set_response_body(e.to_string().as_bytes()).await,
    };
}
