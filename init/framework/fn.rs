use super::*;

fn configure_server_basic(server: &Server) {
    server
        .host(SERVER_HOST)
        .port(SERVER_PORT)
        .set_ttl(SERVER_TTI)
        .set_linger(SERVER_LINGER)
        .set_nodelay(SERVER_NODELAY)
        .error_hook(exception::framework::error_hook)
        .http_buffer(SERVER_HTTP_BUFFER)
        .ws_buffer(SERVER_WS_BUFFER);
}

fn configure_request_middleware(server: &Server) {
    server
        .request_middleware(middleware::request::cross::cross)
        .request_middleware(middleware::request::response::response_header)
        .request_middleware(middleware::request::response::response_status_code)
        .request_middleware(middleware::request::response::response_body);
}

fn configure_response_middleware(server: &Server) {
    server
        .response_middleware(middleware::response::send::send)
        .response_middleware(middleware::response::log::log);
}

fn configure_routes(server: &Server) {
    server
        .route("/", controller::root::handle)
        .route(format!("/hello/{{{NAME_KEY}}}"), controller::hello::handle)
        .route("/websocket", controller::ws::handle)
        .route("/favicon.ico", controller::favicon_ico::handle);
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

#[hyperlane(server)]
async fn create_server() {
    configure_server_basic(&server);
    configure_request_middleware(&server);
    configure_routes(&server);
    configure_response_middleware(&server);
    println_success!("Server initialization successful");
    match server.run() {
        Ok(_) => {
            let host_port: String = format!("{SERVER_HOST}:{SERVER_PORT}");
            println_success!("Server listen in: ", host_port)
        }
        Err(server_error) => println_error!("Server run error: ", server_error),
    }
}

pub fn run() {
    runtime().block_on(hyperlane_plugin::server_manager::create_server_manage(
        create_server,
    ));
}
