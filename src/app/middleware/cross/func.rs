use crate::*;

pub async fn cross(arc_lock_controller_data: ArcRwLockControllerData) {
    let mut controller_data: RwLockWriteControllerData =
        arc_lock_controller_data.get_write_lock().await;
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
