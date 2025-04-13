use super::*;

pub async fn http_line_buffer_size(server: &Server) {
    server
        .http_line_buffer_size(SERVER_HTTP_LINE_BUFFER_SIZE)
        .await;
    println_success!(
        "Server http line buffer size: ",
        SERVER_HTTP_LINE_BUFFER_SIZE,
        SPACE,
        "bytes"
    );
}

pub async fn websocket_buffer_size(server: &Server) {
    server
        .websocket_buffer_size(SERVER_WEB_SOCKET_BUFFER_SIZE)
        .await;
    println_success!(
        "Server websocket buffer size: ",
        SERVER_WEB_SOCKET_BUFFER_SIZE,
        SPACE,
        "bytes"
    );
}
