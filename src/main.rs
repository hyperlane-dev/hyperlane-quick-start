pub(crate) mod app;
pub(crate) mod config;
pub(crate) mod init;
pub(crate) mod plugin;
pub(crate) use hyperlane::*;

#[tokio::main]
async fn main() {
    init::server::run_server().await;
}
