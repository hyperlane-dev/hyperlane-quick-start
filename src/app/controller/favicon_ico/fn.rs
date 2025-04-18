use super::*;

pub async fn handle(ctx: Context) {
    let _ = ctx
        .set_response_header(CONTENT_TYPE, IMAGE_PNG)
        .await
        .set_response_header(CACHE_CONTROL, "public, max-age=3600")
        .await
        .set_response_body(LOGO_IMG)
        .await;
}
