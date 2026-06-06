use hyperlane_plugin::{common::GetOrInit, env::EnvPlugin};

use super::*;

/// Implementation of route printing and server startup methods for `ServerBootstrap`.
impl ServerBootstrap {
    /// Prints all registered static, dynamic, and regex routes to the log.
    ///
    /// # Arguments
    ///
    /// - `&Server`: The server instance whose route matcher to inspect.
    async fn print_route_matcher(server: &Server) {
        let route_matcher: &RouteMatcher = server.get_route_matcher();
        for key in route_matcher.get_static_route().keys() {
            info!("Static route {key}");
        }
        for value in route_matcher.get_dynamic_route().values() {
            for (route_pattern, _) in value {
                info!("Dynamic route {route_pattern}");
            }
        }
        for value in route_matcher.get_regex_route().values() {
            for (route_pattern, _) in value {
                info!("Regex route {route_pattern}");
            }
        }
    }
}

/// Implementation of `BootstrapAsyncInit` for `ServerBootstrap`, configuring and starting the HTTP server.
impl BootstrapAsyncInit for ServerBootstrap {
    #[hyperlane(server: Server)]
    async fn init() -> Self {
        let config: ConfigBootstrap = ConfigBootstrap::init().await;
        server
            .request_config(*config.get_request_config())
            .server_config(config.get_server_config().clone());
        match server.run().await {
            Ok(server_hook) => {
                let env_config: &EnvConfig = EnvPlugin::get_or_init();
                let host_port: String = format!(
                    "{}{COLON}{}",
                    env_config.get_server_host(),
                    env_config.get_server_port()
                );
                Self::print_route_matcher(&server).await;
                info!("Server listen in {host_port}");
                ShutdownPlugin::set(server_hook.get_shutdown_hook());
                server_hook.wait().await;
            }
            Err(server_error) => error!("Server run error {server_error}"),
        }
        Self
    }
}
