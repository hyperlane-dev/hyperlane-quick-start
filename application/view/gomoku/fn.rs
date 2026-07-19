use super::*;

/// Renders the gomoku game page and serves as the OpenAPI documentation endpoint for gomoku routes.
#[utoipa::path(
    get,
    path = "/gomoku",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_gomoku_view() {}
