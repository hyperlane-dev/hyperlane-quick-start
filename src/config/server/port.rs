use super::*;
use crate::*;

pub async fn port(server: &mut Server) {
    server.port(SERVER_PORT).await;
    println_success!("Server port: ", SERVER_PORT);
}
