use crate::*;

pub fn cross(arc_lock_controller_data: ArcRwLockControllerData) {
    let mut controller_data: RwLockWriteControllerData =
        get_rw_lock_write_controller_data(&arc_lock_controller_data);
    let response: &mut Response = controller_data.get_mut_response();
    response
        .set_header(ACCESS_CONTROL_ALLOW_ORIGIN, ANY)
        .set_header(ACCESS_CONTROL_ALLOW_METHODS, GET_POST_OPTIONS)
        .set_header(ACCESS_CONTROL_ALLOW_HEADERS, ANY)
        .set_header(
            CONTENT_TYPE,
            format!("{}; {}", APPLICATION_JSON, CHARSET_UTF_8),
        );
}
