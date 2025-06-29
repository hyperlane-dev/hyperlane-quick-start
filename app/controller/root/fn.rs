use super::*;

#[methods(get, post)]
#[utoipa::path(
    get,
    post,
    path = "/",   
    responses(
        (status = 200, description = "首页", body = String)
    )
)]
#[response_status_code(200)]
pub async fn handle(ctx: Context) {
    let html: String = INDEX_HTML.replace("{{ time }}", &time());
    let _ = ctx.set_response_body(html).await;
}
