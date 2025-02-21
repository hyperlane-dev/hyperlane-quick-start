use crate::*;

pub async fn favicon_ico(arc_lock_controller_data: ArcRwLockControllerData) {
    let data: Vec<u8> = plugin::logo_img::func::get_logo_img();
    {
        let mut controller_data: RwLockWriteControllerData =
            arc_lock_controller_data.get_write_lock().await;
        let response: &mut Response = controller_data.get_mut_response();
        response.set_header(CONTENT_TYPE, IMAGE_PNG);
        response.set_header(CACHE_CONTROL, "public, max-age=3600");
    }
    let send_res: ResponseResult = arc_lock_controller_data.send_response(200, data).await;
    arc_lock_controller_data
        .get_log()
        .await
        .info(format!("Response result => {:?}", send_res), log_handler);
}
