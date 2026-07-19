use super::*;

/// Serves the favicon and serves as the OpenAPI documentation endpoint for favicon routes.
#[utoipa::path(
    get,
    path = "/favicon.ico",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_favicon() {}
