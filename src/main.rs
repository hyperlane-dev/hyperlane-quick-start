use {
    hyperlane_bootstrap::{
        application::{cicd::*, db::*, env::*, logger::*},
        framework::{runtime::*, server::*},
    },
    hyperlane_config::framework::*,
    hyperlane_plugin::process::*,
};

use hyperlane_utils::log::*;

fn main() {
    init_log();
    if let Err(error) = init_env_config() {
        error!("{error}");
    }
    info!("Environment configuration loaded successfully");
    runtime().block_on(async move {
        init_db().await;
        init_cicd().await;
        create(SERVER_PID_FILE_PATH, init_server).await;
    });
}
