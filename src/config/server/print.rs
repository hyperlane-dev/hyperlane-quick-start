use super::*;
use crate::*;

pub async fn print(server: &mut Server) {
    server.inner_print(SERVER_INNER_PRINT).await;
    println_success!("Server inner print: ", SERVER_INNER_PRINT);
}
