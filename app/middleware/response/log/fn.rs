use super::*;

pub async fn log(ctx: Context) {
    let request: String = ctx.get_request().await.get_string();
    let response: String = ctx.get_response().await.get_string();
    if *ctx.get_response().await.get_status_code() == 200 {
        println_success!(request, BR, response);
    } else {
        println_warning!(request, BR, response);
    }
    log_info(format!("{request}{BR}{response}")).await;
}
