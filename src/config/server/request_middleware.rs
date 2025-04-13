use super::*;

pub async fn register(server: &Server) {
    server.request_middleware(request::cross::func::cross).await;
    server
        .request_middleware(request::response::func::response_header)
        .await;
    server
        .request_middleware(request::response::func::response_status_code)
        .await;
    println_success!("Server request middleware initialization completed");
}
