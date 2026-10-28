use super::*;

/// Implementation of `BootstrapAsyncInit` for `ConfigBootstrap`, loading server and request configuration from the environment.
impl BootstrapAsyncInit for ConfigBootstrap {
    /// Initializes the configuration bootstrap by loading server and request configuration from the environment.
    ///
    /// # Returns
    ///
    /// - `Self` - The initialized `ConfigBootstrap` instance containing server and request configs.
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
        if let Some(ttl) = env_config.try_get_server_tti() {
            server_config.set_ttl(Some(ttl));
        }
        if let Some(nodelay) = env_config.try_get_server_nodelay() {
            server_config.set_nodelay(Some(nodelay));
        }
        debug!("Server config {server_config:?}");
        info!("Server initialization successful");
        Self {
            server_config,
            request_config,
        }
    }
}
