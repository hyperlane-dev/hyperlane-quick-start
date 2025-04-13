use super::*;

pub async fn favicon_ico(ctx: Context) {
    let _ = ctx
        .set_response_header(CONTENT_TYPE, IMAGE_PNG)
        .await
        .set_response_header(CACHE_CONTROL, "public, max-age=3600")
        .await
        .set_response_body(get_logo_img())
        .await;
}
