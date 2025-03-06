use crate::*;
use plugin::logo_img::func::*;

pub async fn favicon_ico(controller_data: ControllerData) {
    let data: Vec<u8> = get_logo_img();
    let _ = controller_data
        .set_response_header(CONTENT_TYPE, IMAGE_PNG)
        .await
        .set_response_header(CACHE_CONTROL, "public, max-age=3600")
        .await
        .send_response(200, data)
        .await;
}
