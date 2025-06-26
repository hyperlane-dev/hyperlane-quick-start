use super::*;

async fn http_buffer_size(server: &Server) {
    server.http_buffer_size(SERVER_HTTP_LINE_BUFFER_SIZE).await;
    println_success!(
        "Server http line buffer size: ",
        SERVER_HTTP_LINE_BUFFER_SIZE,
        SPACE,
        "bytes"
    );
}

async fn ws_buffer_size(server: &Server) {
    server.ws_buffer_size(SERVER_WS_BUFFER_SIZE).await;
    println_success!(
        "Server websocket buffer size: ",
        SERVER_WS_BUFFER_SIZE,
        SERVER_WS_BUFFER_SIZE,
        SPACE,
        "bytes"
    );
}

async fn pre_ws_upgrade(server: &Server) {
    server.pre_ws_upgrade(service::ws::pre_ws_upgrade).await;
}

async fn host(server: &Server) {
    server.host(SERVER_HOST).await;
    println_success!("Server host: ", SERVER_HOST);
}

async fn linger(server: &Server) {
    server.set_linger(SERVER_LINGER).await;
    println_success!("Server linger: ", format!("{SERVER_LINGER:?}"));
}

async fn port(server: &Server) {
    server.port(SERVER_PORT).await;
    println_success!("Server port: ", SERVER_PORT);
}

async fn nodelay(server: &Server) {
    server.set_nodelay(SERVER_NODELAY).await;
    println_success!("Server nodelay: ", SERVER_NODELAY);
}

async fn error_handler(server: &Server) {
    server
        .error_handler(|data| {
            println_error!("Server error: ", data);
        })
        .await;
}

async fn ttl(server: &Server) {
    server.set_ttl(SERVER_TTI).await;
    println_success!("Server ttl: ", SERVER_TTI);
}

async fn disable_inner_ws_handle(server: &Server) {
    server.disable_ws_handler("/api/ws").await;
    println_success!("Server inner websocket handle disable completed");
}

async fn register_request_middleware(server: &Server) {
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
    println_success!("Server request middleware initialization completed");
}

async fn register_response_middleware(server: &Server) {
    server
        .response_middleware(middleware::response::send::send)
        .await;
    server
        .response_middleware(middleware::response::log::log)
        .await;
    println_success!("Server response middleware initialization completed");
}

async fn register_route(server: &Server) {
    server.route("/", controller::root::handle).await;
    server.route("/upload", controller::upload::html).await;
    server
        .route("/favicon.ico", controller::favicon_ico::handle)
        .await;
    server
        .route(format!("/hello/{{{NAME_KEY}}}"), controller::hello::handle)
        .await;
    server
        .route(format!("/openapi/openapi.json"), controller::openapi::json)
        .await;
    server
        .route(format!("/openapi/index.html"), controller::openapi::html)
        .await;
    server
        .route(
            format!("/static/{{{UPLOAD_DIR_KEY}}}/{{{UPLOAD_FILE_KEY}}}"),
            controller::upload::static_file,
        )
        .await;
    server
        .route(format!("/{{{WS_DIR_KEY}:^ws.*}}"), controller::ws::html)
        .await;

    server.route("/api/ws", controller::ws::handle).await;
    server
        .route("/api/users/online", controller::users::online_users)
        .await;
    server
        .route("/api/upload/save", controller::upload::save)
        .await;
    server
        .route("/api/upload/register", controller::upload::register)
        .await;
    server
        .route("/api/upload/merge", controller::upload::merge)
        .await;

    println_success!("Server route initialization completed");
}

async fn on_ws_connected(server: &Server) {
    server.on_ws_connected(service::ws::on_connected).await;
}

fn runtime() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(get_thread_count())
        .thread_stack_size(1_048_576)
        .max_blocking_threads(2_048)
        .max_io_events_per_tick(1_024)
        .enable_all()
        .build()
        .unwrap()
}

async fn create_server() {
    let server: Server = Server::new();
    host(&server).await;
    port(&server).await;
    ttl(&server).await;
    linger(&server).await;
    nodelay(&server).await;
    error_handler(&server).await;
    error_handler(&server).await;
    http_buffer_size(&server).await;
    ws_buffer_size(&server).await;
    pre_ws_upgrade(&server).await;
    on_ws_connected(&server).await;
    disable_inner_ws_handle(&server).await;
    register_request_middleware(&server).await;
    register_route(&server).await;
    register_response_middleware(&server).await;
    let host_port: String = format!("{SERVER_HOST}:{SERVER_PORT}");
    println_success!("Server initialization successful");
    let server_result: ServerResult = server.run().await;
    match server_result {
        Ok(_) => println_success!("Server listen in: ", host_port),
        Err(server_error) => println_error!("Server run error: ", server_error),
    }
}

pub fn run() {
    if let Err(e) = init_env_config() {
        println_error!("Failed to initialize environment configuration: ", e);
    }
    println_success!("Environment configuration loaded successfully");
    runtime().block_on(hyperlane_plugin::server_manager::create_server_manage(
        create_server,
    ));
}
