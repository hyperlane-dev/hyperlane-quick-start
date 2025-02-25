use crate::*;

pub async fn index(arc_lock_controller_data: ArcRwLockControllerData) {
    let send_res: ResponseResult = arc_lock_controller_data
        .send_response(200, "hello hyperlane => /index")
        .await;
    arc_lock_controller_data
        .log_info(format!("Response result => {:?}", send_res), log_handler)
        .await;
}
