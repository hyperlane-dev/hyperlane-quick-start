pub(crate) mod app;
pub(crate) mod config;
pub(crate) mod init;
pub(crate) mod plugin;
pub(crate) use hyperlane::*;

fn main() {
    init::server::r#fn::run();
}
