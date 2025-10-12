use super::*;

#[utoipa::path(
    get,
    post,
    path = "/api/redis/list",
    description = "Get all Redis records",
    responses(
        (status = 200, description = "List of Redis records", body = Vec<RedisRecord>)
    )
)]
#[route("/api/redis/list")]
#[prologue_macros(
    methods(get, post),
    request_query("keys" => keys_opt),
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn list_records(ctx: Context) {
    let keys: Vec<String> = match keys_opt {
        Some(k) => k.split(',').map(|s: &str| s.to_string()).collect(),
        None => {
            let response = ApiResponse::<()>::error_with_code(
                ResponseCode::BadRequest,
                "Keys parameter is required",
            );
            ctx.set_response_body(&response.to_json_bytes()).await;
            return;
        }
    };
    match get_all_redis_records(keys).await {
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
    path = "/api/redis/create",
    description = "Create a new Redis record",
    request_body = RedisRecord,
    responses(
        (status = 200, description = "Record created successfully"),
        (status = 400, description = "Invalid request data")
    )
)]
#[route("/api/redis/create")]
#[prologue_macros(
    post,
    request_body_json(record_opt: RedisRecord),
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn create_record(ctx: Context) {
    let record: RedisRecord = match record_opt {
        Ok(data) => data,
        Err(error) => {
            let response =
                ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error.to_string());
            ctx.set_response_body(&response.to_json_bytes()).await;
            return;
        }
    };
    match create_redis_record(record).await {
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
    path = "/api/redis/update",
    description = "Update an existing Redis record",
    request_body = RedisRecord,
    responses(
        (status = 200, description = "Record updated successfully"),
        (status = 400, description = "Invalid request data"),
        (status = 404, description = "Record not found")
    )
)]
#[route("/api/redis/update")]
#[prologue_macros(
    post,
    request_body_json(record_opt: RedisRecord),
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn update_record(ctx: Context) {
    let record: RedisRecord = match record_opt {
        Ok(data) => data,
        Err(error) => {
            let response =
                ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error.to_string());
            ctx.set_response_body(&response.to_json_bytes()).await;
            return;
        }
    };
    match update_redis_record(record).await {
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
    path = "/api/redis/delete",
    description = "Delete a Redis record by key",
    params(
        ("key" = String, Path, description = "Key of the record to delete")
    ),
    responses(
        (status = 200, description = "Record deleted successfully"),
        (status = 404, description = "Record not found")
    )
)]
#[route("/api/redis/delete")]
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
    match delete_redis_record(key).await {
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
