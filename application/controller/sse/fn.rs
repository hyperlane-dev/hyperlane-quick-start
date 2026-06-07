use super::*;

/// OpenAPI documentation endpoint for the Server-Sent Events (SSE) streaming route.
#[utoipa::path(
    get,
    path = "/sse",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_sse() {}
