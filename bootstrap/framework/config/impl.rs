use super::*;

impl BootstrapAsyncInit for ConfigBootstrap {
    #[hyperlane(server_config: ServerConfig)]
    async fn init() -> Self {
        let mut request_config: RequestConfig = RequestConfig::default();
        request_config
            .set_max_body_size(SERVER_REQUEST_MAX_BODY_SIZE)
            .set_read_timeout_ms(SERVER_REQUEST_HTTP_READ_TIMEOUT_MS);
        server_config.set_address(Server::format_bind_address(SERVER_HOST, SERVER_PORT));
        server_config.set_ttl(SERVER_TTI);
        server_config.set_nodelay(SERVER_NODELAY);
        debug!("Server config {server_config:?}");
        info!("Server initialization successful");
        Self {
            server_config,
            request_config,
        }
    }
}
