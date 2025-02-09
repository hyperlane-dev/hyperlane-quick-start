use crate::*;

pub fn favicon_ico(arc_lock_controller_data: ArcRwLockControllerData) {
    let send_res: ResponseResult = send_response(&arc_lock_controller_data, 200, vec![]);
    let controller_data: ControllerData = get_read_controller_data(&arc_lock_controller_data);
    controller_data
        .get_log()
        .info(format!("Response result => {:?}", send_res), log_handler);
}
