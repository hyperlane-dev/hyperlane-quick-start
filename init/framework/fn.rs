use super::*;

async fn configure_server_basic(server: &Server) {
    server.host(SERVER_HOST).await;
    server.port(SERVER_PORT).await;
    server.set_ttl(SERVER_TTI).await;
    server.set_linger(SERVER_LINGER).await;
    server.set_nodelay(SERVER_NODELAY).await;
    server.error_hook(exception::framework::error_hook).await;
    server.http_buffer(SERVER_HTTP_BUFFER).await;
    server.ws_buffer(SERVER_WS_BUFFER).await;
    server.connected_hook(service::chat::connected_hook).await;
    server.disable_ws_hook("/api/chat").await;
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
        .route(format!("/{{{WS_DIR_KEY}:^chat.*}}"), controller::chat::html)
        .await;
    server.route("/api/chat", controller::chat::handle).await;
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
    server.route("/log/info", controller::log::info).await;
    server.route("/log/warn", controller::log::warn).await;
    server.route("/log/error", controller::log::error).await;
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
    configure_server_basic(&server).await;
    configure_request_middleware(&server).await;
    configure_routes(&server).await;
    configure_response_middleware(&server).await;
    println_success!("Server initialization successful");
    match server.run().await {
        Ok(_) => {
            let host_port: String = format!("{SERVER_HOST}:{SERVER_PORT}");
            println_success!("Server listen in: ", host_port)
        }
        Err(server_error) => println_error!("Server run error: ", server_error),
    }
}

pub fn run() {
    if let Err(e) = model::business::chat::init_env_config() {
        println_error!(e);
    }
    println_success!("Environment configuration loaded successfully");
    runtime().block_on(hyperlane_plugin::server_manager::create_server_manage(
        create_server,
    ));
}
