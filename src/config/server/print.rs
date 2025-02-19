use crate::*;

pub async fn print(server: &mut Server) {
    server.print(true).await;
}
