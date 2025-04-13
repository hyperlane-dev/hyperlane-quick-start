use super::*;

pub async fn log_dir(server: &Server) {
    server.log_dir(SERVER_LOG_DIR).await;
    println_success!("Server log dir: ", SERVER_LOG_DIR);
}

pub async fn log_size(server: &Server) {
    server.log_size(SERVER_LOG_SIZE).await;
    println_success!("Server log size: ", SERVER_LOG_SIZE, SPACE, "bytes");
}

pub async fn inner_log(server: &Server) {
    server.inner_log(SERVER_INNER_LOG).await;
    println_success!("Server inner log: ", SERVER_INNER_LOG);
}
