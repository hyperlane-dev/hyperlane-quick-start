use super::*;

#[utoipa::path(
    get,
    path = "/api/server/status",   
    responses(
        (status = 200, description = "Server real-time status SSE stream", body = String)
    )
)]
#[get]
#[response_status_code(200)]
#[route("/api/server/status")]
#[response_header(CONTENT_TYPE => TEXT_EVENT_STREAM)]
pub async fn status_sse(ctx: Context) {
    let _ = ctx.send().await;
    loop {
        let server_status: ServerStatus = get_server_status().await;
        let status_json: String = serde_json::to_string(&server_status).unwrap_or_default();
        let sse_data: String = format!("data: {}\n\n", status_json);
        let send_result: ResponseResult = ctx.set_response_body(sse_data).await.send_body().await;
        if send_result.is_err() {
            break;
        }
        sleep(Duration::from_millis(360)).await;
    }
    let _ = ctx.closed().await;
}

#[utoipa::path(
    get,
    path = "/api/server/info",   
    responses(
        (status = 200, description = "Server system information", body = String)
    )
)]
#[get]
#[route("/api/server/info")]
#[response_status_code(200)]
#[response_header(CONTENT_TYPE => APPLICATION_JSON)]
pub async fn system_info(ctx: Context) {
    let system_info: SystemInfo = get_system_info().await;
    let info_json: String = serde_json::to_string(&system_info).unwrap_or_default();
    ctx.set_response_body(info_json).await;
}

#[utoipa::path(
    get,
    post,
    path = "/monitor",   
    responses(
        (status = 200, description = "Server monitoring dashboard interface", body = String)
    )
)]
#[route("/monitor")]
#[methods(get, post)]
#[response_status_code(200)]
#[response_body(MONITOR_DASHBOARD_HTML)]
pub async fn monitor_dashboard(ctx: Context) {}

#[utoipa::path(
    get,
    path = "/api/network/capture",
    responses(
        (status = 200, description = "Network capture data", body = String)
    )
)]
#[get]
#[route("/api/network/capture")]
pub async fn network_capture_data(ctx: Context) {
    get_network_capture_data(ctx).await;
}

#[utoipa::path(
    get,
    path = "/api/network/capture/stream",
    responses(
        (status = 200, description = "Network capture stream", body = String)
    )
)]
#[get]
#[route("/api/network/capture/stream")]
pub async fn network_capture_stream(ctx: Context) {
    get_network_capture_stream(ctx).await;
}
