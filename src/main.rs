use {
    hyperlane_bootstrap::{
        application::{cicd::*, db::*, env::*, logger::*, monitor::*},
        common::*,
        framework::{runtime::*, server::*},
    },
    hyperlane_config::framework::*,
    hyperlane_plugin::process::*,
};

use hyperlane_utils::log::*;

fn main() {
    LoggerBootstrap::init();
    EnvBootstrap::init();
    info!("Environment configuration loaded successfully");
    RuntimeBootstrap::init().get_runtime().block_on(async move {
        DbBootstrap::init().await;
        CicdBootstrap::init().await;
        MonitorBootstrap::init().await;
        ProcessPlugin::create(SERVER_PID_FILE_PATH, || async {
            ServerBootstrap::init().await;
        })
        .await;
    });
}
