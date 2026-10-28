use super::*;

/// openapi monitor view.
#[utoipa::path(
    get,
    path = ROUTE_MONITOR,
    responses(
        (status = 200, description = HTTP_REASON_SUCCESS),
        (status = 400, description = HTTP_REASON_BAD_REQUEST),
        (status = 404, description = HTTP_REASON_NOT_FOUND),
        (status = 500, description = HTTP_REASON_INTERNAL_SERVER_ERROR)
    )
)]
/// Body of the `openapi_monitor_view` free function.
#[instrument_trace]
pub fn openapi_monitor_view() {}
