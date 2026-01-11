use super::*;

#[utoipa::path(
    get,
    path = "/upload",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_upload_view() {}

#[utoipa::path(
    get,
    path = "/upload/file/{upload_dir}/{upload_file}",
    params(
        ("upload_dir" = String, Path, description = "Upload directory"),
        ("upload_file" = String, Path, description = "Upload file name")
    ),
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_upload_file() {}
