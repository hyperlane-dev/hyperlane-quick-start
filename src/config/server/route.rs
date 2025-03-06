use crate::*;
use app::controller::*;

pub async fn register(server: &mut Server) {
    server.router("/", root::func::root).await;
    server
        .router("/favicon.ico", favicon_ico::func::favicon_ico)
        .await;
}
