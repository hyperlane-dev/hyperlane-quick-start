use {
    hyperlane_bootstrap::{
        application::{db::*, env::*, logger::*},
        common::*,
        framework::{config::*, runtime::*, server::*},
    },
    hyperlane_plugin::{env::EnvConfig, process::*},
};

use hyperlane_utils::log::*;

fn main() {
    EnvBootstrap::init();
    LoggerBootstrap::init();
    EnvConfig::log_config();
    info!("Environment configuration loaded successfully");
    RuntimeBootstrap::init().get_runtime().block_on(async move {
        DbBootstrap::init().await;
        ProcessPlugin::create(ConfigBootstrap::get_server_pid_file_path(), || async {
            ServerBootstrap::init().await;
        })
        .await;
    });
}
