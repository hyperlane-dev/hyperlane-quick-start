use super::*;

pub async fn log(ctx: Context) {
    if ctx.get_request().await.upgrade_type_is_websocket() {
        return;
    }
    let request: String = ctx.get_request().await.get_string();
    log_info(request).await;
}
