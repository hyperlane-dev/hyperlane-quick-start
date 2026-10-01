use super::*;

/// openapi monitor status sse.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_MONITOR_SERVER_STATUS,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_monitor_status_sse() {}

/// openapi monitor system info.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_MONITOR_SERVER_INFO,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS, body = SystemInfo),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_monitor_system_info() {}

/// openapi monitor network capture data.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_MONITOR_NETWORK_CAPTURE,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS, body = NetworkStats),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_monitor_network_capture_data() {}

/// openapi monitor network capture stream.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_MONITOR_NETWORK_CAPTURE_STREAM,
    responses(
        (status = 200, description = OPENAPI_DESCRIPTION_SUCCESS),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_monitor_network_capture_stream() {}

/// openapi monitor performance history.
#[utoipa::path(
    get,
    path = OPENAPI_PATH_MONITOR_SERVER_PERFORMANCE_HISTORY,
    responses(
        (
            status = 200,
            description = OPENAPI_DESCRIPTION_SUCCESS,
            body = PerformanceHistoryResponse
        ),
        (status = 400, description = OPENAPI_DESCRIPTION_BAD_REQUEST),
        (status = 404, description = OPENAPI_DESCRIPTION_NOT_FOUND),
        (status = 500, description = OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR)
    )
)]
#[instrument_trace]
pub fn openapi_monitor_performance_history() {}
