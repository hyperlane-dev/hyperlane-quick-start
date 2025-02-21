use crate::*;

pub async fn response_header(arc_lock_controller_data: ArcRwLockControllerData) {
    let mut controller_data: RwLockWriteControllerData =
        arc_lock_controller_data.get_write_lock().await;
    let response: &mut Response = controller_data.get_mut_response();
    response.set_header(SERVER, config::server::constant::SERVER_NAME);
    response.set_header(CONNECTION, CONNECTION_KEEP_ALIVE);
    let content_type: String = format!("{}{}{}", TEXT_HTML, SEMICOLON_SPACE, CHARSET_UTF_8);
    response.set_header(CONTENT_TYPE, content_type);
}
