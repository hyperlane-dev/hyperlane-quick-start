use super::*;
use crate::service::server_status::{get_server_status, get_system_info};
use tokio::time::{Duration, sleep};

#[get]
#[utoipa::path(
    get,
    path = "/api/server/status",   
    responses(
        (status = 200, description = "服务器实时状态SSE流", body = String)
    )
)]
pub async fn status_sse(ctx: Context) {
    let _ = ctx
        .set_response_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
        .await
        .set_response_header(CACHE_CONTROL, "no-cache")
        .await
        .set_response_header(CONNECTION, "keep-alive")
        .await
        .set_response_header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .await
        .set_response_status_code(200)
        .await
        .send()
        .await;

    loop {
        let server_status = get_server_status().await;
        let status_json = serde_json::to_string(&server_status).unwrap_or_default();

        let sse_data = format!("data: {}\n\n", status_json);

        let send_result = ctx.set_response_body(sse_data).await.send_body().await;

        if send_result.is_err() {
            break;
        }

        sleep(Duration::from_secs(2)).await;
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
    let system_info = get_system_info().await;
    let info_json = serde_json::to_string(&system_info).unwrap_or_default();
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
    let html = include_str!("../../../resources/static/html/monitor.html");
    ctx.set_response_body(html).await;
}
