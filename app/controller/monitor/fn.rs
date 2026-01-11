use super::*;

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
pub fn openapi_monitor_status_sse() {}

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
pub fn openapi_monitor_system_info() {
    trace!("openapi_monitor_system_info");
}

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
pub fn openapi_monitor_network_capture_data() {
    trace!("openapi_monitor_network_capture_data");
}

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
pub fn openapi_monitor_network_capture_stream() {
    trace!("openapi_monitor_network_capture_stream");
}
