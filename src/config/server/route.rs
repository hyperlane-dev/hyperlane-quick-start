use super::*;

pub async fn register(server: &Server) {
    server.route("/", controller::root::func::handle).await;
    server
        .route(
            format!("/hello/:{NAME_KEY}"),
            controller::hello::func::handle,
        )
        .await;
    server
        .route("/websocket", controller::websocket::func::handle)
        .await;
    server
        .route("/favicon.ico", controller::favicon_ico::func::handle)
        .await;
    println_success!("Server route initialization completed");
}
