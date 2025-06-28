use super::*;

pub async fn log(ctx: Context) {
    let request: String = ctx.get_request().await.get_string();
    let response: String = ctx.get_response().await.get_string();
    log_info(format!("{request}{BR}{response}")).await;
}
