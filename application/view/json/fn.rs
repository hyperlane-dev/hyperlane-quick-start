use super::*;

/// Renders the JSON viewer page and serves as the OpenAPI documentation endpoint for JSON routes.
#[utoipa::path(
    get,
    path = "/json",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_json_view() {}
