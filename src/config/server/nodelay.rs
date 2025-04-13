use super::*;

pub async fn nodelay(server: &Server) {
    server.set_nodelay(SERVER_NODELAY).await;
    println_success!("Server nodelay: ", SERVER_NODELAY);
}
