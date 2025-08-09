use super::*;

pub async fn create<F, Fut>(server_hook: F)
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let args: Vec<String> = args().collect();
    let mut manager: ServerManager = ServerManager::new();
    manager
        .set_pid_file(PID_FILE_PATH)
        .set_server_hook(server_hook);
    let is_daemon: bool = args.len() >= 3 && args[2].to_lowercase() == "-d";
    let start_server = || async {
        if is_daemon {
            match manager.start_daemon().await {
                Ok(_) => println_success!("Server started in background successfully"),
                Err(e) => println_error!(format!("Error starting server in background: {e}")),
            };
        } else {
            println_success!("Server started successfully");
            manager.start().await;
        }
    };
    let stop_server = || async {
        match manager.stop().await {
            Ok(_) => println_success!("Server stopped successfully"),
            Err(e) => println_error!(format!("Error stopping server: {e}")),
        };
    };
    let hot_restart_server = || async {
        match manager
            .watch_detached(&["--clear", "--skip-local-deps", "-q", "-x", "run"])
            .await
        {
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
