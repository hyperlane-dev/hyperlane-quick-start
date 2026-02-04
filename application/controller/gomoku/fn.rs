use super::*;

#[utoipa::path(
    get,
    path = "/api/gomoku",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_gomoku_websocket() {}
