use super::*;

pub async fn log(ctx: Context) {
    let request: String = ctx.get_request().await.get_string();
    let response: String = ctx.get_response().await.get_string();
    ctx.async_log_info(&request, log_handler)
        .await
        .async_log_info(&response, log_handler)
        .await;
}
