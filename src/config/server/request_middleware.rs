use crate::*;
use app::middleware::request::*;

pub async fn register(server: &Server) {
    server.request_middleware(cross::func::cross).await;
    server
        .request_middleware(response::func::response_header)
        .await;
    server
        .request_middleware(response::func::response_status_code)
        .await;
    println_success!("Server request middleware initialization completed");
}
