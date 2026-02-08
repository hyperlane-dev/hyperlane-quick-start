use super::*;

impl BootstrapAsyncInit for ConfigBootstrap {
    async fn init() -> Self {
        let server: Server = Server::default();
        let request_config: RequestConfig = RequestConfig::default();
        let server_config: ServerConfig = ServerConfig::default();
        request_config
            .max_body_size(SERVER_REQUEST_MAX_BODY_SIZE)
            .await
            .http_read_timeout_ms(SERVER_REQUEST_HTTP_READ_TIMEOUT_MS)
            .await;
        server_config.host(SERVER_HOST).await;
        server_config.port(SERVER_PORT).await;
        server_config.ttl(SERVER_TTI).await;
        server_config.nodelay(SERVER_NODELAY).await;
        server.server_config(server_config.clone()).await;
        server.request_config(request_config).await;
        debug!("Server config {server_config:?}");
        info!("Server initialization successful");
        Self
    }
}
