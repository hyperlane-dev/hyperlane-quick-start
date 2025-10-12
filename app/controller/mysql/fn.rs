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
#[prologue_macros(
    methods(get, post),
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn list_records(ctx: Context) {
    match get_all_mysql_records().await {
        Ok(records) => {
            let response = ApiResponse::success(records);
            ctx.set_response_body(&response.to_json_bytes()).await
        }
        Err(error) => {
            let response = ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
            ctx.set_response_body(&response.to_json_bytes()).await
        }
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
#[prologue_macros(
    post,
    request_body_json(record_opt: MysqlRecord),
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn create_record(ctx: Context) {
    let record: MysqlRecord = match record_opt {
        Ok(data) => data,
        Err(error) => {
            let response =
                ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error.to_string());
            ctx.set_response_body(&response.to_json_bytes()).await;
            return;
        }
    };
    match create_mysql_record(record).await {
        Ok(_) => {
            let response = ApiResponse::<()>::success_without_data("Record created successfully");
            ctx.set_response_body(&response.to_json_bytes()).await
        }
        Err(error) => {
            let response = ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
            ctx.set_response_body(&response.to_json_bytes()).await
        }
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
#[prologue_macros(
    post,
    request_body_json(record_opt: MysqlRecord),
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn update_record(ctx: Context) {
    let record: MysqlRecord = match record_opt {
        Ok(data) => data,
        Err(error) => {
            let response =
                ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error.to_string());
            ctx.set_response_body(&response.to_json_bytes()).await;
            return;
        }
    };
    match update_mysql_record(record).await {
        Ok(_) => {
            let response = ApiResponse::<()>::success_without_data("Record updated successfully");
            ctx.set_response_body(&response.to_json_bytes()).await
        }
        Err(error) => {
            let response = ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
            ctx.set_response_body(&response.to_json_bytes()).await
        }
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
#[prologue_macros(
    post,
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn delete_record(ctx: Context) {
    let querys: RequestQuerys = ctx.get_request_querys().await;
    let key: &String = match querys.get("key") {
        Some(k) => k,
        None => {
            let response = ApiResponse::<()>::error_with_code(
                ResponseCode::BadRequest,
                "Key parameter is required",
            );
            ctx.set_response_body(&response.to_json_bytes()).await;
            return;
        }
    };
    match delete_mysql_record(key).await {
        Ok(_) => {
            let response = ApiResponse::<()>::success_without_data("Record deleted successfully");
            ctx.set_response_body(&response.to_json_bytes()).await
        }
        Err(error) => {
            let response = ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
            ctx.set_response_body(&response.to_json_bytes()).await
        }
    };
}
