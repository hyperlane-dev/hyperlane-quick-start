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
    response_header(CONTENT_TYPE => TEXT_PLAIN)
)]
pub async fn list_records(ctx: Context) {
    let keys: Vec<String> = match keys_opt {
        Some(k) => k.split(',').map(|s: &str| s.to_string()).collect(),
        None => {
            ctx.set_response_body("Keys parameter is required").await;
            return;
        }
    };
    match get_all_redis_records(keys).await {
        Ok(records) => {
            let data: ResponseBody = serde_json::to_vec(&records).unwrap_or_default();
            ctx.set_response_body(&data).await
        }
        Err(error) => ctx.set_response_body(&error).await,
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
    response_header(CONTENT_TYPE => TEXT_PLAIN)
)]
pub async fn create_record(ctx: Context) {
    let record: RedisRecord = match record_opt {
        Ok(data) => data,
        Err(error) => {
            ctx.set_response_body(&error.to_string()).await;
            return;
        }
    };
    match create_redis_record(record).await {
        Ok(_) => ctx.set_response_body("Record created successfully").await,
        Err(error) => ctx.set_response_body(&error).await,
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
    response_header(CONTENT_TYPE => TEXT_PLAIN)
)]
pub async fn update_record(ctx: Context) {
    let record: RedisRecord = match record_opt {
        Ok(data) => data,
        Err(error) => {
            ctx.set_response_body(&error.to_string()).await;
            return;
        }
    };
    match update_redis_record(record).await {
        Ok(_) => ctx.set_response_body("Record updated successfully").await,
        Err(error) => ctx.set_response_body(&error).await,
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
    response_header(CONTENT_TYPE => TEXT_PLAIN)
)]
pub async fn delete_record(ctx: Context) {
    let querys: RequestQuerys = ctx.get_request_querys().await;
    let key: &String = match querys.get("key") {
        Some(k) => k,
        None => {
            ctx.set_response_body("Key parameter is required").await;
            return;
        }
    };
    match delete_redis_record(key).await {
        Ok(_) => ctx.set_response_body("Record deleted successfully").await,
        Err(error) => ctx.set_response_body(&error).await,
    };
}
