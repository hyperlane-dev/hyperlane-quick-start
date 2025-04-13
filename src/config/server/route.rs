use super::*;
use app::controller::{hello::r#const::NAME_KEY, *};

pub async fn register(server: &Server) {
    server.route("/", root::func::handle).await;
    server
        .route(format!("/hello/:{NAME_KEY}"), hello::func::handle)
        .await;
    server.route("/websocket", websocket::func::handle).await;
    server
        .route("/favicon.ico", favicon_ico::func::favicon_ico)
        .await;
    println_success!("Server route initialization completed");
}
