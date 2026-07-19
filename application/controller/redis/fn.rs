use super::*;

/// openapi redis records.
#[utoipa::path(
    get,
    path = "/api/redis/list",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_redis_records() {}

/// openapi redis record create.
#[utoipa::path(
    post,
    path = "/api/redis/create",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_redis_record_create() {}

/// openapi redis record update.
#[utoipa::path(
    post,
    path = "/api/redis/update",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_redis_record_update() {}

/// openapi redis record delete.
#[utoipa::path(
    post,
    path = "/api/redis/delete",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_redis_record_delete() {}
