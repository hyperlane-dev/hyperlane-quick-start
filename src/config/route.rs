use crate::*;

pub async fn route(server: &mut Server) {
    server.router("/", app::controller::root::root);
    server.router("/index", app::controller::index::index);
    server.router("/favicon.ico", app::controller::favicon_ico::favicon_ico);
}
