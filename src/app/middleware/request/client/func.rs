use crate::*;

pub async fn client(controller_data: ControllerData) {
    let socket_addr: OptionSocketAddr = controller_data.get_socket_addr().await;
    let socket_addr_string: String = socket_addr.unwrap_or(DEFAULT_SOCKET_ADDR).to_string();
    controller_data
        .set_response_header("SocketAddr", socket_addr_string)
        .await;
}
