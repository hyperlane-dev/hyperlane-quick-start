use super::*;
use crate::*;

pub async fn host(server: &mut Server) {
    server.host(SERVER_HOST).await;
    println_success!("host: ", SERVER_HOST);
}
