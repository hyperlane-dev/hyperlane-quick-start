use super::*;

#[route("/api/server/status")]
#[utoipa::path(
    get,
    path = "/api/server/status",
    responses(
        (status = 200, description = "Server real-time status SSE stream", body = String)
    )
)]
#[get]
#[response_status_code(200)]
#[response_header(CONTENT_TYPE => TEXT_EVENT_STREAM)]
pub async fn status_sse(ctx: Context) {
    let _ = ctx.send().await;
    loop {
        let server_status: ServerStatus = get_server_status().await;
        let status_json: String = serde_json::to_string(&server_status).unwrap_or_default();
        let sse_data: String = format!("data: {}\n\n", status_json);
        let send_result: ResponseResult = ctx.set_response_body(&sse_data).await.send_body().await;
        if send_result.is_err() {
            break;
        }
        sleep(Duration::from_millis(360)).await;
    }
    let _ = ctx.closed().await;
}

#[route("/api/server/info")]
#[utoipa::path(
    get,
    path = "/api/server/info",
    responses(
        (status = 200, description = "Server system information", body = String)
    )
)]
#[prologue_macros(
    get,
    response_status_code(200),
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn system_info(ctx: Context) {
    let system_info: SystemInfo = get_system_info().await;
    let info_json: ResponseBody = serde_json::to_vec(&system_info).unwrap_or_default();
    ctx.set_response_body(&info_json).await;
}

#[route("/monitor")]
#[utoipa::path(
    get,
    post,
    path = "/monitor",
    responses(
        (status = 200, description = "Server monitoring dashboard interface", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(200),
    response_body(MONITOR_DASHBOARD_HTML)
)]
pub async fn monitor_dashboard(ctx: Context) {}

#[route("/api/network/capture")]
#[utoipa::path(
    get,
    post,
    path = "/api/network/capture",
    responses(
        (status = 200, description = "Network capture data", body = String)
    )
)]
#[methods(get, post)]
pub async fn network_capture_data(ctx: Context) {
    get_network_capture_data(ctx).await;
}

#[route("/api/network/capture/stream")]
#[utoipa::path(
    get,
    post,
    path = "/api/network/capture/stream",
    responses(
        (status = 200, description = "Network capture stream", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    response_header(CONTENT_TYPE => TEXT_EVENT_STREAM),
    response_header(CACHE_CONTROL => NO_CACHE),
    response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)
)]
pub async fn network_capture_stream(ctx: Context) {
    get_network_capture_stream(ctx).await;
}
