use super::*;

pub async fn log(ctx: Context) {
    let request_body_len: usize = ctx.get_request().await.get_body().len();
    ctx.set_request_body(format!("binary data len: {request_body_len}"))
        .await;
    let request: String = ctx.get_request().await.get_string();
    let response: String = ctx.get_response().await.get_string();
    ctx.async_log_info(&request, log_handler)
        .await
        .async_log_info(&response, log_handler)
        .await;
}
