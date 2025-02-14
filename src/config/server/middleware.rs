use crate::*;

pub async fn middleware(server: &mut Server) {
    server.middleware(app::middleware::cross::func::cross);
    server.middleware(app::middleware::response_header::func::response_header);
    server.middleware(app::middleware::client::func::client);
}
