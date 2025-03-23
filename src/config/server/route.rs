use crate::*;
use app::controller::*;

pub async fn register(server: &Server) {
    server.route("/", root::func::handle).await;
    server.route("/websocket", websocket::func::handle).await;
    server
        .route("/favicon.ico", favicon_ico::func::favicon_ico)
        .await;
    println_success!("Server route initialization completed");
}
