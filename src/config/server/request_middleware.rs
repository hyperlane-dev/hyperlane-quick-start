use crate::*;
use app::middleware::request::*;

pub async fn register(server: &mut Server) {
    server.request_middleware(cross::func::cross).await;
    server
        .request_middleware(response_header::func::response_header)
        .await;
    server.request_middleware(client::func::client).await;
}
