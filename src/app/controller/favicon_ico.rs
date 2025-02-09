use crate::*;

pub async fn favicon_ico(arc_lock_controller_data: ArcRwLockControllerData) {
    let data: Vec<u8> = plugin::logo_img::get_logo_img();
    {
        let mut controller_data: RwLockWriteControllerData =
            get_rw_lock_write_controller_data(&arc_lock_controller_data);
        let response: &mut Response = controller_data.get_mut_response();
        response.set_header(CONTENT_TYPE, IMAGE_PNG);
    }
    let send_res: ResponseResult = send_response(&arc_lock_controller_data, 200, data);
    get_controller_data(&arc_lock_controller_data)
        .get_log()
        .info(format!("Response result => {:?}", send_res), log_handler);
}
