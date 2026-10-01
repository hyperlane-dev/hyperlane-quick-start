use super::*;

/// openapi gomoku websocket.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_GOMOKU,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_gomoku_websocket() {}
