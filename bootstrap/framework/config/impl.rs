use super::*;

impl BootstrapAsyncInit for ConfigBootstrap {
    #[hyperlane(server_config: ServerConfig)]
    async fn init() -> Self {
        let request_config: RequestConfig = RequestConfig::default();
        request_config
            .max_body_size(SERVER_REQUEST_MAX_BODY_SIZE)
            .await
            .http_read_timeout_ms(SERVER_REQUEST_HTTP_READ_TIMEOUT_MS)
            .await;
        server_config.host(SERVER_HOST).await;
        server_config.port(SERVER_PORT).await;
        server_config.ttl(SERVER_TTI).await;
        server_config.nodelay(SERVER_NODELAY).await;
        debug!("Server config {server_config:?}");
        info!("Server initialization successful");
        Self {
            server_config,
            request_config,
        }
    }
}
