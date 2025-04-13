use super::*;

pub async fn port(server: &Server) {
    server.port(SERVER_PORT).await;
    println_success!("Server port: ", SERVER_PORT);
}
