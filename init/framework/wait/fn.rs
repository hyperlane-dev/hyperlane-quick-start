use super::*;
use rustls;

#[hyperlane(config: ServerConfig)]
async fn init_config(server: &Server) {
    let mut request_config: RequestConfig = RequestConfig::default();
    request_config
        .set_max_body_size(MAX_BODY_SIZE)
        .set_http_read_timeout_ms(HTTP_READ_TIMEOUT_MS);
    config.host(SERVER_HOST).await;
    config.port(SERVER_PORT).await;
    config.ttl(SERVER_TTI).await;
    config.linger(SERVER_LINGER).await;
    config.nodelay(SERVER_NODELAY).await;
    config.request_config(request_config).await;
    server.config(config).await;
}

async fn print_route_matcher(server: &Server) {
    let route_matcher: RouteMatcher = server.get_route_matcher().await;
    for key in route_matcher.get_static_route().keys() {
        println_success!("Static route: {key}");
    }
    for value in route_matcher.get_dynamic_route().values() {
        for (route_pattern, _) in value {
            println_success!("Dynamic route: {route_pattern}");
        }
    }
    for value in route_matcher.get_regex_route().values() {
        for (route_pattern, _) in value {
            println_success!("Regex route: {route_pattern}");
        }
    }
}

async fn init_network_capture() {
    MonitorService::start_network_capture().await;
}

async fn init_db() {
    let env: &EnvConfig = get_global_env_config();
    if *env.get_enable_mysql() {
        let _: Result<sea_orm::DatabaseConnection, String> = connection_mysql_db().await;
    }
    if *env.get_enable_postgresql() {
        let _: Result<sea_orm::DatabaseConnection, String> = connection_postgresql_db().await;
    }
    if *env.get_enable_redis() {
        let _: Result<std::sync::Arc<hyperlane_utils::redis::Connection>, String> =
            connection_redis_db().await;
    }
    match initialize_auto_creation().await {
        Ok(_) => {
            println_success!("Auto-creation initialization successful");
        }
        Err(error) => {
            println_error!("Auto-creation initialization failed: {error}");
        }
    };
}

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

#[hyperlane(server: Server)]
async fn create_server() {
    init_config(&server).await;
    init_network_capture().await;
    init_db().await;
    println_success!("Server initialization successful");
    let server_result: Result<ServerControlHook, ServerError> = server.run().await;
    match server_result {
        Ok(server_hook) => {
            let host_port: String = format!("{SERVER_HOST}:{SERVER_PORT}");
            print_route_matcher(&server).await;
            println_success!("Server listen in: {host_port}");
            let shutdown: SharedAsyncTaskFactory<()> = server_hook.get_shutdown_hook().clone();
            set_shutdown(shutdown);
            server_hook.wait().await;
        }
        Err(server_error) => println_error!("Server run error: {server_error}"),
    }
}

pub fn run() {
    // Initialize the crypto provider to fix rustls issue
    if let Err(e) = rustls::crypto::ring::default_provider().install_default() {
        println_warning!("Failed to install default crypto provider: {:?}, trying fallback...", e);
        // Fallback to aws-lc-rs if ring is not available
        if let Err(e) = rustls::crypto::aws_lc_rs::default_provider().install_default() {
            println_error!("Failed to install aws-lc-rs crypto provider: {:?}", e);
        }
    }
    
    if let Err(error) = init_env_config() {
        println_error!("{error}");
    }
    println_success!("Environment configuration loaded successfully");
    runtime().block_on(create(create_server));
}
