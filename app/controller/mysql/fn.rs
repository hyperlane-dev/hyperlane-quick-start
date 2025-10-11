use super::*;

#[utoipa::path(
    get,
    path = "/api/mysql/delete",
    description = "Get all MySQL records",
    responses(
        (status = 200, description = "List of MySQL records", body = Vec<MysqlRecord>)
    )
)]
#[route("/api/mysql/delete")]
#[prologue_macros(get)]
pub async fn get_records(ctx: Context) {
    match get_all_mysql_records().await {
        Ok(records) => {
            let data: ResponseBody = serde_json::to_vec(&records).unwrap_or_default();
            ctx.set_response_body(&data).await
        }
        Err(e) => ctx.set_response_body(&e).await,
    };
}

#[utoipa::path(
    post,
    path = "/api/mysql/create",
    description = "Create a new MySQL record",
    request_body = MysqlRecord,
    responses(
        (status = 200, description = "Record created successfully"),
        (status = 400, description = "Invalid request data")
    )
)]
#[route("/api/mysql/create")]
#[prologue_macros(post)]
pub async fn create_record(ctx: Context) {
    let record: MysqlRecord = match ctx.get_request_body_json().await {
        Ok(r) => r,
        Err(e) => {
            ctx.set_response_body(&e.to_string()).await;
            return;
        }
    };
    match create_mysql_record(record).await {
        Ok(_) => ctx.set_response_body("Record created successfully").await,
        Err(e) => ctx.set_response_body(&e).await,
    };
}

#[utoipa::path(
    put,
    path = "/api/mysql/list",
    description = "Update an existing MySQL record",
    request_body = MysqlRecord,
    responses(
        (status = 200, description = "Record updated successfully"),
        (status = 400, description = "Invalid request data"),
        (status = 404, description = "Record not found")
    )
)]
#[route("/api/mysql/list")]
#[prologue_macros(put)]
pub async fn update_record(ctx: Context) {
    let record: MysqlRecord = match ctx.get_request_body_json().await {
        Ok(r) => r,
        Err(e) => {
            ctx.set_response_body(&e.to_string()).await;
            return;
        }
    };
    match update_mysql_record(record).await {
        Ok(_) => ctx.set_response_body("Record updated successfully").await,
        Err(e) => ctx.set_response_body(&e).await,
    };
}

#[utoipa::path(
    delete,
    path = "/api/mysql/update",
    description = "Delete a MySQL record by key",
    params(
        ("key" = String, Path, description = "Key of the record to delete")
    ),
    responses(
        (status = 200, description = "Record deleted successfully"),
        (status = 404, description = "Record not found")
    )
)]
#[route("/api/mysql/update")]
#[prologue_macros(delete)]
pub async fn delete_record(ctx: Context) {
    let querys: RequestQuerys = ctx.get_request_querys().await;
    let key: &String = match querys.get("key") {
        Some(k) => k,
        None => {
            ctx.set_response_body("Key parameter is required").await;
            return;
        }
    };
    match delete_mysql_record(key).await {
        Ok(_) => ctx.set_response_body("Record deleted successfully").await,
        Err(e) => ctx.set_response_body(&e).await,
    };
}
