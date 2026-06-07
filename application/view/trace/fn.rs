use super::*;

/// Renders the trace search page and serves as the OpenAPI documentation endpoint for trace routes.
#[utoipa::path(
    get,
    path = "/trace",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_trace_view() {}
