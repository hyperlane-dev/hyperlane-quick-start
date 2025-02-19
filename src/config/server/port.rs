use crate::*;

pub async fn port(server: &mut Server) {
    server.port(60000).await;
}
