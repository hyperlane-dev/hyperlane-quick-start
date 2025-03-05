use crate::*;

pub async fn favicon_ico(controller_data: ControllerData) {
    let data: Vec<u8> = plugin::logo_img::func::get_logo_img();
    let send_res: ResponseResult = controller_data
        .set_response_header(CONTENT_TYPE, IMAGE_PNG)
        .await
        .set_response_header(CACHE_CONTROL, "public, max-age=3600")
        .await
        .send_response(200, data)
        .await;
    let request: Request = controller_data.get_request().await;
    controller_data
        .log_info(format!("Request result => {}", request), log_handler)
        .await
        .log_info(format!("Response result => {:?}", send_res), log_handler)
        .await;
}
