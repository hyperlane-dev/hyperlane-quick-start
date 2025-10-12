use super::*;

#[route("/api/server/status")]
#[utoipa::path(
    get,
    path = "/api/server/status",
    description = "Stream server real-time status via Server-Sent Events",
    responses(
        (status = 200, description = "Successfully streaming server status", body = String)
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
        let sse_data: String = format!("data: {status_json}{DOUBLE_BR}");
        let send_result: ResponseResult = ctx.set_response_body(&sse_data).await.send_body().await;
        if send_result.is_err() {
            break;
        }
        sleep(Duration::from_millis(1000)).await;
    }
    let _ = ctx.closed().await;
}

#[route("/api/server/info")]
#[utoipa::path(
    get,
    path = "/api/server/info",
    description = "Get server system information",
    responses(
        (status = 200, description = "Successfully retrieved server system information", body = String)
    )
)]
#[prologue_macros(
    get,
    response_status_code(200),
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn system_info(ctx: Context) {
    let system_info: SystemInfo = get_system_info().await;
    let response = ApiResponse::success(system_info);
    ctx.set_response_body(&response.to_json_bytes()).await;
}

#[route("/api/network/capture")]
#[utoipa::path(
    get,
    post,
    path = "/api/network/capture",
    description = "Get network capture data",
    responses(
        (status = 200, description = "Successfully retrieved network capture data", body = String)
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
    description = "Stream network capture data",
    responses(
        (status = 200, description = "Successfully streaming network capture data", body = String)
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
