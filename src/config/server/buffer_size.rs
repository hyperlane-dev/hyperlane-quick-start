use super::*;
use crate::*;

pub async fn websocket_buffer_size(server: &mut Server) {
    server
        .websocket_buffer_size(SERVER_WEB_SOCKET_BUFFER_SIZE)
        .await;
    println_success!("Server websocket_buffer size: ", SERVER_WEB_SOCKET_BUFFER_SIZE);
}
