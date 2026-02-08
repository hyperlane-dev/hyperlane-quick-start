use {
    hyperlane_bootstrap::{
        application::{db::*, env::*, logger::*},
        framework::{runtime::*, server::*},
    },
    hyperlane_config::framework::*,
    hyperlane_plugin::process::*,
};

use hyperlane_utils::log::*;

fn main() {
    LoggerBootstrap::init();
    if let Err(error) = EnvBootstrap::init() {
        error!("{error}");
    }
    info!("Environment configuration loaded successfully");
    RuntimeBootstrap::init().block_on(async move {
        DbBootstrap::init().await;
        ProcessPlugin::create(SERVER_PID_FILE_PATH, ServerBootstrap::init).await;
    });
}
