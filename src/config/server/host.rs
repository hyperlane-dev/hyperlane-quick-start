use crate::*;

pub async fn host(server: &mut Server) {
    server.host("0.0.0.0").await;
}
