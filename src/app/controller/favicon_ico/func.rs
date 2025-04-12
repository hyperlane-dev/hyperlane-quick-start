use crate::*;
use plugin::logo_img::func::*;

pub async fn favicon_ico(ctx: Context) {
    let data: Vec<u8> = get_logo_img();
    let _ = ctx
        .set_response_header(CONTENT_TYPE, IMAGE_PNG)
        .await
        .set_response_header(CACHE_CONTROL, "public, max-age=3600")
        .await
        .set_response_body(data)
        .await;
}
