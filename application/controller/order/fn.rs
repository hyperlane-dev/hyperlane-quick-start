use super::*;

/// openapi record create.
#[utoipa::path(
    post,
    path = "/api/order/record/create",
    responses(
        (status = 200, description = "Record created successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_record_create() {}

/// openapi record list.
#[utoipa::path(
    get,
    path = "/api/order/record/list",
    responses(
        (status = 200, description = "List of records retrieved successfully"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_record_list() {}

/// openapi record get.
#[utoipa::path(
    get,
    path = "/api/order/record/get/{id}",
    params(
        ("id" = i32, Path, description = "Record ID")
    ),
    responses(
        (status = 200, description = "Record details retrieved successfully"),
        (status = 400, description = "Bad request"),
        (status = 404, description = "Record not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_record_get() {}

/// openapi overview statistics.
#[utoipa::path(
    get,
    path = "/api/order/overview/statistics",
    responses(
        (status = 200, description = "Statistics retrieved successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_overview_statistics() {}

/// openapi image upload.
#[utoipa::path(
    post,
    path = "/api/order/image/upload",
    responses(
        (status = 200, description = "Image uploaded successfully"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_image_upload() {}

/// openapi image list.
#[utoipa::path(
    get,
    path = "/api/order/image/list/{record_id}",
    params(
        ("record_id" = i32, Path, description = "Record ID")
    ),
    responses(
        (status = 200, description = "Image list retrieved successfully"),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_image_list() {}

/// openapi image download.
#[utoipa::path(
    get,
    path = "/api/order/image/download/{id}",
    params(
        ("id" = i32, Path, description = "Image ID")
    ),
    responses(
        (status = 200, description = "Image downloaded successfully"),
        (status = 404, description = "Image not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[instrument_trace]
pub fn openapi_image_download() {}
