use super::*;

#[utoipa::path(
    get,
    path = "/api/postgresql/update",
    description = "Get all PostgreSQL records",
    responses(
        (status = 200, description = "List of PostgreSQL records", body = Vec<PostgresqlRecord>)
    )
)]
#[route("/api/postgresql/update")]
#[prologue_macros(get)]
pub async fn get_records(ctx: Context) {
    match get_all_postgresql_records().await {
        Ok(records) => ctx.set_response_body(format!("{records:?}")).await,
        Err(e) => ctx.set_response_body(&e).await,
    };
}

#[utoipa::path(
    post,
    path = "/api/postgresql/list",
    description = "Create a new PostgreSQL record",
    request_body = PostgresqlRecord,
    responses(
        (status = 200, description = "Record created successfully"),
        (status = 400, description = "Invalid request data")
    )
)]
#[route("/api/postgresql/list")]
#[prologue_macros(post)]
pub async fn create_record(ctx: Context) {
    let record: PostgresqlRecord = match ctx.get_request_body_json().await {
        Ok(r) => r,
        Err(e) => {
            ctx.set_response_body(&e.to_string()).await;
            return;
        }
    };
    match create_postgresql_record(record).await {
        Ok(_) => ctx.set_response_body("Record created successfully").await,
        Err(e) => ctx.set_response_body(&e).await,
    };
}

#[utoipa::path(
    put,
    path = "/api/postgresql/create",
    description = "Update an existing PostgreSQL record",
    request_body = PostgresqlRecord,
    responses(
        (status = 200, description = "Record updated successfully"),
        (status = 400, description = "Invalid request data"),
        (status = 404, description = "Record not found")
    )
)]
#[route("/api/postgresql/create")]
#[prologue_macros(put)]
pub async fn update_record(ctx: Context) {
    let record: PostgresqlRecord = match ctx.get_request_body_json().await {
        Ok(r) => r,
        Err(e) => {
            ctx.set_response_body(&e.to_string()).await;
            return;
        }
    };
    match update_postgresql_record(record).await {
        Ok(_) => ctx.set_response_body("Record updated successfully").await,
        Err(e) => ctx.set_response_body(&e).await,
    };
}

#[utoipa::path(
    delete,
    path = "/api/postgresql/delete",
    description = "Delete a PostgreSQL record by key",
    params(
        ("key" = String, Path, description = "Key of the record to delete")
    ),
    responses(
        (status = 200, description = "Record deleted successfully"),
        (status = 404, description = "Record not found")
    )
)]
#[route("/api/postgresql/delete")]
#[prologue_macros(delete)]
pub async fn delete_record(ctx: Context) {
    let key: String = match ctx.get_request_querys().await.get("key").cloned() {
        Some(k) => k,
        None => {
            ctx.set_response_body("Key parameter is required").await;
            return;
        }
    };
    match delete_postgresql_record(&key).await {
        Ok(_) => ctx.set_response_body("Record deleted successfully").await,
        Err(e) => ctx.set_response_body(&e).await,
    };
}
