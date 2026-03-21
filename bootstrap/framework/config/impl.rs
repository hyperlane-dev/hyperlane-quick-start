use super::*;

impl BootstrapAsyncInit for ConfigBootstrap {
    #[hyperlane(server_config: ServerConfig)]
    async fn init() -> Self {
        let env_config: &EnvConfig = EnvPlugin::get_or_init();
        let mut request_config: RequestConfig = RequestConfig::default();
        request_config
            .set_max_body_size(env_config.get_server_request_max_body_size())
            .set_read_timeout_ms(env_config.get_server_request_http_read_timeout_ms());
        server_config.set_address(Server::format_bind_address(
            env_config.get_server_host(),
            env_config.get_server_port(),
        ));
        server_config.set_ttl(env_config.get_server_tti());
        server_config.set_nodelay(env_config.get_server_nodelay());
        debug!("Server config {server_config:?}");
        info!("Server initialization successful");
        Self {
            server_config,
            request_config,
        }
    }
}
