use super::*;

pub async fn ttl(server: &Server) {
    server.set_ttl(SERVER_TTI).await;
    println_success!("Server ttl: ", SERVER_TTI);
}
