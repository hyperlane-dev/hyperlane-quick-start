use crate::*;

pub async fn response_header(arc_lock_controller_data: ArcRwLockControllerData) {
    let content_type: String = format!("{}{}{}", TEXT_HTML, SEMICOLON_SPACE, CHARSET_UTF_8);
    arc_lock_controller_data
        .set_response_header(SERVER, config::server::constant::SERVER_NAME)
        .await
        .set_response_header(CONNECTION, CONNECTION_KEEP_ALIVE)
        .await
        .set_response_header(CONTENT_TYPE, content_type)
        .await;
}
