use super::*;

#[server_config(config)]
async fn configure_config(server: &Server) {
    config.host(SERVER_HOST).await;
    config.port(SERVER_PORT).await;
    config.ttl(SERVER_TTI).await;
    config.linger(SERVER_LINGER).await;
    config.nodelay(SERVER_NODELAY).await;
    config.http_buffer(SERVER_HTTP_BUFFER).await;
    config.ws_buffer(SERVER_WS_BUFFER).await;
    server.config(config).await;
}

async fn configure_panic_hook(server: &Server) {
    server.panic_hook(exception::framework::panic_hook).await;
}

async fn configure_request_middleware(server: &Server) {
    server
        .request_middleware(middleware::request::cross::cross)
        .await;
    server
        .request_middleware(middleware::request::response::response_header)
        .await;
    server
        .request_middleware(middleware::request::response::response_status_code)
        .await;
    server
        .request_middleware(middleware::request::response::response_body)
        .await;
}

async fn configure_response_middleware(server: &Server) {
    server
        .response_middleware(middleware::response::send::send)
        .await;
    server
        .response_middleware(middleware::response::log::log)
        .await;
}

async fn configure_routes(server: &Server) {
    server.route("/", controller::root::handle).await;
    server
        .route(format!("/hello/{{{NAME_KEY}}}"), controller::hello::handle)
        .await;
    server.route("/websocket", controller::ws::handle).await;
    server
        .route("/favicon.ico", controller::favicon_ico::handle)
        .await;
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

#[server(server)]
async fn create_server() {
    configure_config(&server).await;
    configure_panic_hook(&server).await;
    configure_request_middleware(&server).await;
    configure_routes(&server).await;
    configure_response_middleware(&server).await;
    println_success!("Server initialization successful");
    let server_result: ServerResult<ServerHook> = server.run().await;
    match server_result {
        Ok(server_hook) => {
            let host_port: String = format!("{SERVER_HOST}:{SERVER_PORT}");
            println_success!("Server listen in: ", host_port);
            let shutdown: ArcFnPinBoxFutureSend<()> = server_hook.get_shutdown_hook().clone();
            set_shutdown(shutdown);
            server_hook.wait().await;
        }
        Err(server_error) => println_error!("Server run error: ", server_error),
    }
}

pub fn run() {
    runtime().block_on(server_manager::create(create_server));
}
