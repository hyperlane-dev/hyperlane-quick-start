use super::*;

/// Renders the upload page and serves as the OpenAPI documentation endpoint for upload routes.
#[utoipa::path(
    get,
    path = ROUTE_UPLOAD,
    responses(
        (status = 200, description = HTTP_REASON_SUCCESS),
        (status = 400, description = HTTP_REASON_BAD_REQUEST),
        (status = 404, description = HTTP_REASON_NOT_FOUND),
        (status = 500, description = HTTP_REASON_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_upload_view() {}

/// Serves the file upload endpoint and serves as the OpenAPI documentation endpoint for file upload routes.
#[utoipa::path(
    get,
    path = ROUTE_UPLOAD_FILE,
    params(
        ("upload_dir" = String, Path, description = ROUTE_PARAM_DESC_UPLOAD_DIR),
        ("upload_file" = String, Path, description = ROUTE_PARAM_DESC_UPLOAD_FILE)
    ),
    responses(
        (status = 200, description = HTTP_REASON_SUCCESS),
        (status = 400, description = HTTP_REASON_BAD_REQUEST),
        (status = 404, description = HTTP_REASON_NOT_FOUND),
        (status = 500, description = HTTP_REASON_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_upload_file() {}
