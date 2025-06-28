use super::*;

#[get]
#[utoipa::path(
    get,
    path = "/favicon.ico",   
    responses(
        (status = 200, description = "图标", body = String)
    )
)]
#[status_code(200)]
pub async fn handle(ctx: Context) {
    let _ = ctx
        .set_response_header(CONTENT_TYPE, IMAGE_PNG)
        .await
        .set_response_header(CACHE_CONTROL, "public, max-age=3600")
        .await
        .set_response_body(LOGO_IMG)
        .await;
}
