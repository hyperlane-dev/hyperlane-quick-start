use super::*;

#[get]
#[utoipa::path(
    get,
    path = "/api/server/status",   
    responses(
        (status = 200, description = "服务器实时状态SSE流", body = String)
    )
)]
#[response_status_code(200)]
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

#[get]
#[utoipa::path(
    get,
    path = "/api/server/info",   
    responses(
        (status = 200, description = "服务器系统信息", body = String)
    )
)]
#[response_status_code(200)]
#[response_header(CONTENT_TYPE => APPLICATION_JSON)]
pub async fn system_info(ctx: Context) {
    let system_info: SystemInfo = get_system_info().await;
    let info_json: String = serde_json::to_string(&system_info).unwrap_or_default();
    ctx.set_response_body(info_json).await;
}

#[methods(get, post)]
#[utoipa::path(
    get,
    post,
    path = "/monitor",   
    responses(
        (status = 200, description = "服务器监控大屏界面", body = String)
    )
)]
#[response_status_code(200)]
pub async fn monitor_dashboard(ctx: Context) {
    let html: &str = include_str!("../../../resources/static/html/monitor_parallax.html");
    ctx.set_response_body(html).await;
}

#[get]
#[utoipa::path(
    get,
    path = "/api/network/capture",
    responses(
        (status = 200, description = "Network capture data", body = String)
    )
)]
pub async fn network_capture_data(ctx: Context) {
    get_network_capture_data(ctx).await;
}

#[get]
#[utoipa::path(
    get,
    path = "/api/network/capture/stream",
    responses(
        (status = 200, description = "Network capture stream", body = String)
    )
)]
pub async fn network_capture_stream(ctx: Context) {
    get_network_capture_stream(ctx).await;
}

pub async fn init_network_capture() {
    start_network_capture().await;
}
