use super::*;

pub async fn host(server: &Server) {
    server.host(SERVER_HOST).await;
    println_success!("Server host: ", SERVER_HOST);
}
