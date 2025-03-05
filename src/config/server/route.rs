use crate::*;

pub async fn route(server: &mut Server) {
    server.router("/", app::controller::root::func::root).await;
    server
        .router(
            "/favicon.ico",
            app::controller::favicon_ico::func::favicon_ico,
        )
        .await;
}
