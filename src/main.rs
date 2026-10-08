#![recursion_limit = "1024"]

use {
    hyperlane_bootstrap::{
        application::{db::*, env::*, logger::*},
        common::*,
        framework::{runtime::*, server::*},
    },
    hyperlane_plugin::{common::GetOrInit, env::*, process::*},
};

use hyperlane_utils::log::*;

/// Entry point for the Hyperlane server application.
///
/// Initializes the environment configuration, logger, runtime, database connections,
/// and starts the server with process management.
fn main() {
    EnvBootstrap::init();
    LoggerBootstrap::init();
    EnvConfig::log_config();
    info!("Environment configuration loaded successfully");
    let env_config: &EnvConfig = EnvPlugin::get_or_init();
    RuntimeBootstrap::init().get_runtime().block_on(async move {
        DbBootstrap::init().await;
        ProcessPlugin::create(env_config.get_server_pid_file_path(), || async {
            ServerBootstrap::init().await;
        })
        .await;
    });
}
