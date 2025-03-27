use crate::*;

pub async fn log(ctx: Context) {
    let request: String = ctx.get_request().await.get_string();
    let response: String = ctx.get_response().await.get_string();
    ctx.log_info(&request, log_handler)
        .await
        .log_info(&response, log_handler)
        .await;
}
