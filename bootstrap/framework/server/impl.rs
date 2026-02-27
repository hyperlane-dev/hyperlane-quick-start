use super::*;

impl ServerBootstrap {
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

impl BootstrapAsyncInit for ServerBootstrap {
    #[hyperlane(server: Server)]
    async fn init() -> Self {
        let config: ConfigBootstrap = ConfigBootstrap::init().await;
        server
            .request_config(*config.get_request_config())
            .server_config(config.get_server_config().clone());
        match server.run().await {
            Ok(server_hook) => {
                let host_port: String = format!("{SERVER_HOST}{COLON}{SERVER_PORT}");
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
