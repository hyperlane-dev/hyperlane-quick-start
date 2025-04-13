use super::*;

pub async fn print(server: &Server) {
    server.inner_print(SERVER_INNER_PRINT).await;
    println_success!("Server inner print: ", SERVER_INNER_PRINT);
}
