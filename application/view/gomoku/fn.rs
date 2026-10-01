use super::*;

/// Renders the gomoku game page and serves as the OpenAPI documentation endpoint for gomoku routes.
#[utoipa::path(
    get,
    path = ROUTE_GOMOKU,
    responses(
        (status = 200, description = HTTP_REASON_SUCCESS),
        (status = 400, description = HTTP_REASON_BAD_REQUEST),
        (status = 404, description = HTTP_REASON_NOT_FOUND),
        (status = 500, description = HTTP_REASON_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_gomoku_view() {}
