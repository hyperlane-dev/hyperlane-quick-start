use super::*;

#[hyperlane(config: ServerConfig)]
async fn configure_config(server: &Server) {
    config.host(SERVER_HOST).await;
    config.port(SERVER_PORT).await;
    config.ttl(SERVER_TTI).await;
    config.linger(SERVER_LINGER).await;
    config.nodelay(SERVER_NODELAY).await;
    config.buffer(SERVER_BUFFER).await;
    server.config(config).await;
}

async fn init_network_capture() {
    start_network_capture().await;
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
    configure_config(&server).await;
    init_network_capture().await;
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
    if let Err(e) = model::business::chat::init_env_config() {
        println_error!(e);
    }
    println_success!("Environment configuration loaded successfully");
    runtime().block_on(process::create(create_server));
}
