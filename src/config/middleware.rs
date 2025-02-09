use crate::*;

pub async fn middleware(server: &mut Server) {
    server.middleware(app::middleware::cross::cross);
    server.middleware(app::middleware::response_header::response_header);
}
