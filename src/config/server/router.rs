use crate::*;
use app::controller::*;

pub async fn register(server: &mut Server) {
    server.router("/", root::func::handle).await;
    server.router("/websocket", websocket::func::handle).await;
    server
        .router("/favicon.ico", favicon_ico::func::favicon_ico)
        .await;
    println_success!("Server router initialization completed");
}
