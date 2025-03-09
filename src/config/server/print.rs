use super::*;
use crate::*;

pub async fn print(server: &mut Server) {
    server.print(SERVER_PRINT).await;
    println_success!("Server print: ", SERVER_PRINT);
}
