use crate::*;

pub async fn response_header(controller_data: ControllerData) {
    let socket_addr_string: String = controller_data.get_socket_addr_or_default_string().await;
    let content_type: String = content_type_charset(TEXT_PLAIN, UTF8);
    controller_data
        .set_response_header(SERVER, HYPERLANE)
        .await
        .set_response_header(CONNECTION, CONNECTION_KEEP_ALIVE)
        .await
        .set_response_header(CONTENT_TYPE, content_type)
        .await
        .set_response_header(DATE, current_date_gmt())
        .await
        .set_response_header("SocketAddr", socket_addr_string)
        .await;
}

pub async fn response_status_code(controller_data: ControllerData) {
    controller_data.set_response_status_code(200).await;
}
