use super::*;

#[instrument_trace]
fn runtime() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(num_cpus::get_physical() << 1)
        .thread_stack_size(1_048_576)
        .max_blocking_threads(2_048)
        .max_io_events_per_tick(1_024)
        .enable_all()
        .build()
        .unwrap()
}

#[hyperlane(config: ServerConfig)]
#[instrument_trace]
async fn init_server_config(server: &Server) {
    let mut request_config: RequestConfig = RequestConfig::default();
    request_config
        .set_max_body_size(SERVER_REQUEST_MAX_BODY_SIZE)
        .set_http_read_timeout_ms(SERVER_REQUEST_HTTP_READ_TIMEOUT_MS);
    config.host(SERVER_HOST).await;
    config.port(SERVER_PORT).await;
    config.ttl(SERVER_TTI).await;
    config.nodelay(SERVER_NODELAY).await;
    config.request_config(request_config).await;
    server.config(config.clone()).await;
    debug!("Server config{COLON_SPACE}{:?}", config);
    info!("Server initialization successful");
}

#[instrument_trace]
async fn print_route_matcher(server: &Server) {
    let route_matcher: RouteMatcher = server.get_route_matcher().await;
    for key in route_matcher.get_static_route().keys() {
        info!("Static route{COLON_SPACE}{key}");
    }
    for value in route_matcher.get_dynamic_route().values() {
        for (route_pattern, _) in value {
            info!("Dynamic route{COLON_SPACE}{route_pattern}");
        }
    }
    for value in route_matcher.get_regex_route().values() {
        for (route_pattern, _) in value {
            info!("Regex route{COLON_SPACE}{route_pattern}");
        }
    }
}

#[hyperlane(server: Server)]
#[instrument_trace]
async fn create_server() {
    init_server_config(&server).await;
    match server.run().await {
        Ok(server_hook) => {
            let host_port: String = format!("{SERVER_HOST}{COLON}{SERVER_PORT}");
            print_route_matcher(&server).await;
            info!("Server listen in{COLON_SPACE}{host_port}");
            let shutdown: SharedAsyncTaskFactory<()> = server_hook.get_shutdown_hook().clone();
            set_shutdown(shutdown);
            server_hook.wait().await;
        }
        Err(server_error) => error!("Server run error{COLON_SPACE}{server_error}"),
    }
}

#[instrument_trace]
pub fn run() {
    init_log();
    runtime().block_on(create(create_server));
}
