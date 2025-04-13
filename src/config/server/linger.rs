use super::*;

pub async fn linger(server: &Server) {
    server.set_linger(SERVER_LINGER).await;
    println_success!("Server linger: ", format!("{:?}", SERVER_LINGER));
}
