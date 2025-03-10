use super::*;
use crate::*;

pub async fn log_dir(server: &mut Server) {
    server.log_dir(SERVER_LOG_DIR).await;
    println_success!("Server log dir: ", SERVER_LOG_DIR);
}

pub async fn log_size(server: &mut Server) {
    server.log_size(SERVER_LOG_SIZE).await;
    println_success!("Server log size: ", SERVER_LOG_SIZE, SPACE, "bytes");
}

pub async fn log_interval_millis(server: &mut Server) {
    server.log_interval_millis(SERVER_LOG_INTERVAL_MILLIS).await;
    println_success!(
        "Server log interval millis: ",
        SERVER_LOG_INTERVAL_MILLIS,
        SPACE,
        "ms"
    );
}
