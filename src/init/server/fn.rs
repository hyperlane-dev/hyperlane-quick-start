use super::*;

async fn http_line_buffer_size(server: &Server) {
    server
        .http_line_buffer_size(SERVER_HTTP_LINE_BUFFER_SIZE)
        .await;
    println_success!(
        "Server http line buffer size: ",
        SERVER_HTTP_LINE_BUFFER_SIZE,
        SPACE,
        "bytes"
    );
}

async fn websocket_buffer_size(server: &Server) {
    server
        .websocket_buffer_size(SERVER_WEB_SOCKET_BUFFER_SIZE)
        .await;
    println_success!(
        "Server websocket buffer size: ",
        SERVER_WEB_SOCKET_BUFFER_SIZE,
        SPACE,
        "bytes"
    );
}

async fn host(server: &Server) {
    server.host(SERVER_HOST).await;
    println_success!("Server host: ", SERVER_HOST);
}

async fn linger(server: &Server) {
    server.set_linger(SERVER_LINGER).await;
    println_success!("Server linger: ", format!("{:?}", SERVER_LINGER));
}

async fn log_dir(server: &Server) {
    server.log_dir(SERVER_LOG_DIR).await;
    println_success!("Server log dir: ", SERVER_LOG_DIR);
}

async fn log_size(server: &Server) {
    server.log_size(SERVER_LOG_SIZE).await;
    println_success!("Server log size: ", SERVER_LOG_SIZE, SPACE, "bytes");
}

async fn inner_log(server: &Server) {
    server.inner_log(SERVER_INNER_LOG).await;
    println_success!("Server inner log: ", SERVER_INNER_LOG);
}

async fn port(server: &Server) {
    server.port(SERVER_PORT).await;
    println_success!("Server port: ", SERVER_PORT);
}

async fn nodelay(server: &Server) {
    server.set_nodelay(SERVER_NODELAY).await;
    println_success!("Server nodelay: ", SERVER_NODELAY);
}

async fn print(server: &Server) {
    server.inner_print(SERVER_INNER_PRINT).await;
    println_success!("Server inner print: ", SERVER_INNER_PRINT);
}

async fn ttl(server: &Server) {
    server.set_ttl(SERVER_TTI).await;
    println_success!("Server ttl: ", SERVER_TTI);
}

async fn register_request_middleware(server: &Server) {
    server.request_middleware(request::cross::r#fn::cross).await;
    server
        .request_middleware(request::response::r#fn::response_header)
        .await;
    server
        .request_middleware(request::response::r#fn::response_status_code)
        .await;
    println_success!("Server request middleware initialization completed");
}

async fn register_response_middleware(server: &Server) {
    server.response_middleware(response::send::r#fn::send).await;
    server.response_middleware(response::log::r#fn::log).await;
    println_success!("Server response middleware initialization completed");
}

async fn register_route(server: &Server) {
    server.route("/", controller::root::r#fn::handle).await;
    server
        .route(
            format!("/hello/:{NAME_KEY}"),
            controller::hello::r#fn::handle,
        )
        .await;
    server
        .route("/websocket", controller::websocket::r#fn::handle)
        .await;
    server
        .route("/favicon.ico", controller::favicon_ico::r#fn::handle)
        .await;
    println_success!("Server route initialization completed");
}

fn runtime() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(get_thread_count())
        .thread_stack_size(2097152)
        .max_blocking_threads(5120)
        .max_io_events_per_tick(5120)
        .enable_all()
        .build()
        .unwrap()
}

async fn create_server() {
    let server: Server = Server::new();
    host(&server).await;
    port(&server).await;
    print(&server).await;
    log_dir(&server).await;
    log_size(&server).await;
    inner_log(&server).await;
    ttl(&server).await;
    linger(&server).await;
    nodelay(&server).await;
    http_line_buffer_size(&server).await;
    websocket_buffer_size(&server).await;
    register_request_middleware(&server).await;
    register_route(&server).await;
    register_response_middleware(&server).await;
    let host_port: String = format!("{}:{}", SERVER_HOST, SERVER_PORT);
    println_success!("Server initialization successful");
    let server_result: ServerResult = server.listen().await;
    match server_result {
        Ok(_) => println_success!("Server listen in: ", host_port),
        Err(server_error) => println_error!("Server run error: ", server_error),
    }
}

pub fn run() {
    runtime().block_on(plugin::server_manager::r#fn::create_server_manage(
        create_server,
    ));
}
