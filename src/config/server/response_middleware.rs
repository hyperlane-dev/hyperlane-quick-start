use crate::*;
use app::middleware::response::*;

pub async fn register(server: &mut Server) {
    server.response_middleware(log::func::log).await;
}
