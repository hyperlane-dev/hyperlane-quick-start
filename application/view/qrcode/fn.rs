use super::*;

/// openapi qrcode view.
#[utoipa::path(
    get,
    path = "/qrcode",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_qrcode_view() {}
