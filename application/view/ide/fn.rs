use super::*;

/// openapi ide view.
#[utoipa::path(
    get,
    path = "/ide",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_ide_view() {}
