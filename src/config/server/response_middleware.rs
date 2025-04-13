use super::*;

pub async fn register(server: &Server) {
    server.response_middleware(response::send::func::send).await;
    server.response_middleware(response::log::func::log).await;
    println_success!("Server response middleware initialization completed");
}
