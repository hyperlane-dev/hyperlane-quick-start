use super::*;

/// openapi monitor status sse.
#[utoipa::path(
    get,
    path = "/api/server/status",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_monitor_status_sse() {}

/// openapi monitor system info.
#[utoipa::path(
    get,
    path = "/api/server/info",
    responses(
        (status = 200, description = "Success", body = SystemInfo),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_monitor_system_info() {}

/// openapi monitor network capture data.
#[utoipa::path(
    get,
    path = "/api/network/capture",
    responses(
        (status = 200, description = "Success", body = NetworkStats),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_monitor_network_capture_data() {}

/// openapi monitor network capture stream.
#[utoipa::path(
    get,
    path = "/api/network/capture/stream",
    responses(
        (status = 200, description = "Success"),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_monitor_network_capture_stream() {}

/// openapi monitor performance history.
#[utoipa::path(
    get,
    path = "/api/server/performance/history",
    responses(
        (status = 200, description = "Success", body = PerformanceHistoryResponse),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Internal Server Error")
    )
)]
#[instrument_trace]
pub fn openapi_monitor_performance_history() {}
