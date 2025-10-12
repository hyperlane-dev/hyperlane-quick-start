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

async fn init_network_capture() {
    start_network_capture().await;
}

async fn init_db() {
    let env: &EnvConfig = get_global_env_config();
    if env.enable_mysql {
        let _ = connection_mysql_db().await;
    }
    if env.enable_postgresql {
        let _ = connection_postgresql_db().await;
    }
    if env.enable_redis {
        let _ = connection_redis_db().await;
    }
    match initialize_auto_creation().await {
        Ok(_) => {
            println_success!("Auto-creation initialization successful");
        }
        Err(error) => {
            println_error!(format!("Auto-creation initialization failed: {error}"));
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
    if let Err(error) = init_env_config() {
        println_error!(error);
    }
    println_success!("Environment configuration loaded successfully");
    runtime().block_on(create(create_server));
}
