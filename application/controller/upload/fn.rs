use super::*;

#[utoipa::path(
    post,
    path = "/api/upload/register",
    request_body = FileChunkData,
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_upload_register() {}

#[utoipa::path(
    post,
    path = "/api/upload/save",
    request_body = FileChunkData,
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_upload_save() {}

#[utoipa::path(
    post,
    path = "/api/upload/merge",
    request_body = FileChunkData,
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_upload_merge() {}
