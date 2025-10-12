use super::*;

#[utoipa::path(
    get,
    post,
    path = "/api/mysql/list",
    description = "Get all MySQL records",
    responses(
        (status = 200, description = "List of MySQL records", body = Vec<MysqlRecord>)
    )
)]
#[route("/api/mysql/list")]
#[prologue_macros(methods(get, post))]
pub async fn list_records(ctx: Context) {
    match get_all_mysql_records().await {
        Ok(records) => {
            let data: ResponseBody = serde_json::to_vec(&records).unwrap_or_default();
            ctx.set_response_body(&data).await
        }
        Err(error) => ctx.set_response_body(&error).await,
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
#[prologue_macros(post, request_body_json(record_opt: MysqlRecord))]
pub async fn create_record(ctx: Context) {
    let record: MysqlRecord = match record_opt {
        Ok(data) => data,
        Err(error) => {
            ctx.set_response_body(&error.to_string()).await;
            return;
        }
    };
    match create_mysql_record(record).await {
        Ok(_) => ctx.set_response_body("Record created successfully").await,
        Err(error) => ctx.set_response_body(&error).await,
    };
}

#[utoipa::path(
    post,
    path = "/api/mysql/update",
    description = "Update an existing MySQL record",
    request_body = MysqlRecord,
    responses(
        (status = 200, description = "Record updated successfully"),
        (status = 400, description = "Invalid request data"),
        (status = 404, description = "Record not found")
    )
)]
#[route("/api/mysql/update")]
#[prologue_macros(post, request_body_json(record_opt: MysqlRecord))]
pub async fn update_record(ctx: Context) {
    let record: MysqlRecord = match record_opt {
        Ok(data) => data,
        Err(error) => {
            ctx.set_response_body(&error.to_string()).await;
            return;
        }
    };
    match update_mysql_record(record).await {
        Ok(_) => ctx.set_response_body("Record updated successfully").await,
        Err(error) => ctx.set_response_body(&error).await,
    };
}

#[utoipa::path(
    post,
    path = "/api/mysql/delete",
    description = "Delete a MySQL record by key",
    params(
        ("key" = String, Path, description = "Key of the record to delete")
    ),
    responses(
        (status = 200, description = "Record deleted successfully"),
        (status = 404, description = "Record not found")
    )
)]
#[route("/api/mysql/delete")]
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
        Err(error) => ctx.set_response_body(&error).await,
    };
}
