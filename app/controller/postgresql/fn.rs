use super::*;

#[utoipa::path(
    get,
    post,
    path = "/api/postgresql/list",
    description = "Get all PostgreSQL records",
    responses(
        (status = 200, description = "List of PostgreSQL records", body = Vec<PostgresqlRecord>)
    )
)]
#[route("/api/postgresql/list")]
#[prologue_macros(
    methods(get, post),
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn list_records(ctx: Context) {
    match get_all_postgresql_records().await {
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
    path = "/api/postgresql/create",
    description = "Create a new PostgreSQL record",
    request_body = PostgresqlRecord,
    responses(
        (status = 200, description = "Record created successfully"),
        (status = 400, description = "Invalid request data")
    )
)]
#[route("/api/postgresql/create")]
#[prologue_macros(
    post,
    request_body_json(record_opt: PostgresqlRecord),
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn create_record(ctx: Context) {
    let record: PostgresqlRecord = match record_opt {
        Ok(data) => data,
        Err(error) => {
            let response =
                ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error.to_string());
            ctx.set_response_body(&response.to_json_bytes()).await;
            return;
        }
    };
    match create_postgresql_record(record).await {
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
    path = "/api/postgresql/update",
    description = "Update an existing PostgreSQL record",
    request_body = PostgresqlRecord,
    responses(
        (status = 200, description = "Record updated successfully"),
        (status = 400, description = "Invalid request data"),
        (status = 404, description = "Record not found")
    )
)]
#[route("/api/postgresql/update")]
#[prologue_macros(
    post,
    request_body_json(record_opt: PostgresqlRecord),
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn update_record(ctx: Context) {
    let record: PostgresqlRecord = match record_opt {
        Ok(data) => data,
        Err(error) => {
            let response =
                ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error.to_string());
            ctx.set_response_body(&response.to_json_bytes()).await;
            return;
        }
    };
    match update_postgresql_record(record).await {
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
#[prologue_macros(
    post,
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn delete_record(ctx: Context) {
    let key: String = match ctx.get_request_querys().await.get("key").cloned() {
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
    match delete_postgresql_record(&key).await {
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
