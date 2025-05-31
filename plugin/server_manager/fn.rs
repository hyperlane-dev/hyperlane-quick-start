use super::*;

pub async fn create_server_manage<F, Fut>(server_func: F)
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    let args: Vec<String> = args().collect();
    let config: ServerManagerConfig = ServerManagerConfig {
        pid_file: PID_FILE_PATH.to_owned(),
    };
    let manager: ServerManager<F> = ServerManager::new(config, server_func);
    let is_daemon: bool = args.len() >= 3 && args[2].to_lowercase() == "-d";
    let start_server = || async {
        if is_daemon {
            match manager.start_daemon() {
                Ok(_) => println_success!("Server started in background successfully"),
                Err(e) => println_error!(format!("Error starting server in background: {e}")),
            }
        } else {
            println_success!("Server started successfully");
            manager.start().await;
        }
    };
    let stop_server = || async {
        match manager.stop() {
            Ok(_) => println_success!("Server stopped successfully"),
            Err(e) => println_error!(format!("Error stopping server: {e}")),
        }
    };
    let hot_restart_server = || async {
        match manager.hot_restart(&["--clear", "--skip-local-deps", "-q", "-x", "run"]) {
            Ok(_) => println_success!("Server started successfully"),
            Err(e) => println_error!(format!("Error starting server in background: {e}")),
        }
    };
    let restart_server = || async {
        stop_server().await;
        start_server().await;
    };
    if args.len() < 2 {
        start_server().await;
        return;
    }
    let command: String = args[1].to_lowercase();
    match command.as_str() {
        "start" => start_server().await,
        "stop" => stop_server().await,
        "restart" => restart_server().await,
        "hot" => hot_restart_server().await,
        _ => {
            println_error!(format!("Invalid command: {command}"));
        }
    }
}
