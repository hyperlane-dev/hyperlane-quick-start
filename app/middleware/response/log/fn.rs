use super::*;

#[response_middleware(2)]
pub async fn log(ctx: Context) {
    let request: String = ctx.get_request().await.get_string();
    let response: String = ctx.get_response().await.get_string();
    log_info(request).await;
    log_info(response).await
}
