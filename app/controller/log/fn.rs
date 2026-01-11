use super::*;

#[utoipa::path(
    get,
    path = "/log/info",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub fn openapi_log_info() {
    trace!("openapi_log_info");
}

#[utoipa::path(
    get,
    path = "/log/warn",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub fn openapi_log_warn() {
    trace!("openapi_log_warn");
}

#[utoipa::path(
    get,
    path = "/log/error",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub fn openapi_log_error() {
    trace!("openapi_log_error");
}
