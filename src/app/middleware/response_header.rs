use crate::*;

pub fn response_header(arc_lock_controller_data: ArcRwLockControllerData) {
    let mut controller_data: RwLockWriteControllerData =
        get_rw_lock_write_controller_data(&arc_lock_controller_data);
    let response: &mut Response = controller_data.get_mut_response();
    response.set_header("server", "hyperlane");
}
