use super::*;

#[hyperlane(config: ServerConfig)]
#[instrument_trace]
pub async fn init_server_config(server: &Server) {
    let request_config: RequestConfig = RequestConfig::default();
    request_config
        .max_body_size(SERVER_REQUEST_MAX_BODY_SIZE)
        .await
        .http_read_timeout_ms(SERVER_REQUEST_HTTP_READ_TIMEOUT_MS)
        .await;
    config.host(SERVER_HOST).await;
    config.port(SERVER_PORT).await;
    config.ttl(SERVER_TTI).await;
    config.nodelay(SERVER_NODELAY).await;
    server.server_config(config.clone()).await;
    server.request_config(request_config).await;
    debug!("Server config{COLON_SPACE}{:?}", config);
    info!("Server initialization successful");
}
