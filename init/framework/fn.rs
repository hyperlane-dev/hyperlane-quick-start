use super::*;

async fn configure_server_basic(server: &Server) {
    server
        .host(SERVER_HOST)
        .await
        .port(SERVER_PORT)
        .await
        .set_ttl(SERVER_TTI)
        .await
        .set_linger(SERVER_LINGER)
        .await
        .set_nodelay(SERVER_NODELAY)
        .await
        .error_handler(exception::framework::error_handler)
        .await
        .http_buffer_size(SERVER_HTTP_BUFFER_SIZE)
        .await
        .ws_buffer_size(SERVER_WS_BUFFER_SIZE)
        .await
        .on_ws_connected(service::chat::on_connected)
        .await;
}

async fn configure_request_middleware(server: &Server) {
    server
        .request_middleware(middleware::request::cross::cross)
        .await
        .request_middleware(middleware::request::response::response_header)
        .await
        .request_middleware(middleware::request::response::response_status_code)
        .await
        .request_middleware(middleware::request::response::response_body)
        .await;
}

async fn configure_response_middleware(server: &Server) {
    server
        .response_middleware(middleware::response::send::send)
        .await
        .response_middleware(middleware::response::log::log)
        .await;
}

async fn configure_routes(server: &Server) {
    server
        .route("/", controller::root::handle)
        .await
        .route("/upload", controller::upload::html)
        .await
        .route("/favicon.ico", controller::favicon_ico::handle)
        .await
        .route(format!("/hello/{{{NAME_KEY}}}"), controller::hello::handle)
        .await
        .route(format!("/openapi/openapi.json"), controller::openapi::json)
        .await
        .route(format!("/openapi/index.html"), controller::openapi::html)
        .await
        .route(
            format!("/static/{{{UPLOAD_DIR_KEY}}}/{{{UPLOAD_FILE_KEY}}}"),
            controller::upload::static_file,
        )
        .await
        .route(format!("/{{{WS_DIR_KEY}:^chat.*}}"), controller::chat::html)
        .await;

    server
        .route("/api/chat", controller::chat::handle)
        .await
        .route("/api/users/online", controller::users::online_users)
        .await
        .route("/api/upload/save", controller::upload::save)
        .await
        .route("/api/upload/register", controller::upload::register)
        .await
        .route("/api/upload/merge", controller::upload::merge)
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

#[hyperlane(server)]
async fn create_server() {
    configure_server_basic(&server).await;
    configure_request_middleware(&server).await;
    configure_routes(&server).await;
    configure_response_middleware(&server).await;
    println_success!("Server initialization successful");
    let server_run_result: ServerResult = server.run().await;
    match server_run_result {
        Ok(_) => {
            let host_port: String = format!("{SERVER_HOST}:{SERVER_PORT}");
            println_success!("Server listen in: ", host_port)
        }
        Err(server_error) => println_error!("Server run error: ", server_error),
    }
}

pub fn run() {
    if let Err(e) = model::business::chat::init_env_config() {
        println_error!("Failed to initialize environment configuration: ", e);
    }
    println_success!("Environment configuration loaded successfully");
    runtime().block_on(hyperlane_plugin::server_manager::create_server_manage(
        create_server,
    ));
}
