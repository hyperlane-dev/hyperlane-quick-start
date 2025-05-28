use super::*;

pub async fn log(ctx: Context) {
    if ctx.get_request().await.upgrade_type_is_websocket() {
        return;
    }
    let request_body_len: usize = ctx.get_request().await.get_body().len();
    ctx.set_request_body(format!("binary data len: {request_body_len}"))
        .await;
    let request: String = ctx.get_request().await.get_string();
    let response: String = ctx.get_response().await.get_string();
    log_info(request).await;
    log_info(response).await
}
