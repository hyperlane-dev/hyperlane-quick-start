use super::*;

#[hyperlane(config: ServerConfig)]
async fn init_config(server: &Server) {
    config.host(SERVER_HOST).await;
    config.port(SERVER_PORT).await;
    config.ttl(SERVER_TTI).await;
    config.linger(SERVER_LINGER).await;
    config.nodelay(SERVER_NODELAY).await;
    config.buffer(SERVER_BUFFER).await;
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
    println_success!("Server initialization successful");
    let server_result: ServerResult<ServerControlHook> = server.run().await;
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
    runtime().block_on(create(create_server));
}
