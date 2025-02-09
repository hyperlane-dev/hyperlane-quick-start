pub(crate) mod app;
pub(crate) mod config;
pub(crate) mod plugin;
pub(crate) use hyperlane::*;

#[tokio::main]
async fn main() {
    config::server::server().await;
}
