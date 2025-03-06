use crate::*;
use config::server::constant::*;

pub async fn response_header(controller_data: ControllerData) {
    let content_type: String = format!("{}{}{}", TEXT_HTML, SEMICOLON_SPACE, CHARSET_UTF_8);
    controller_data
        .set_response_header(SERVER, SERVER_NAME)
        .await
        .set_response_header(CONNECTION, CONNECTION_KEEP_ALIVE)
        .await
        .set_response_header(CONTENT_TYPE, content_type)
        .await;
}
