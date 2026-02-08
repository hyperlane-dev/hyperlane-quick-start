use {
    hyperlane_bootstrap::{
        application::{db::*, env::*, logger::*},
        framework::{runtime::*, server::*},
    },
    hyperlane_config::framework::*,
    hyperlane_plugin::{common::*, process::*},
};

use hyperlane_utils::log::*;

fn main() {
    LoggerBootstrap::init();
    EnvBootstrap::init();
    info!("Environment configuration loaded successfully");
    RuntimeBootstrap::init().get_runtime().block_on(async move {
        DbBootstrap::init().await;
        ProcessPlugin::create(SERVER_PID_FILE_PATH, || async {
            ServerBootstrap::init().await;
        })
        .await;
    });
}
