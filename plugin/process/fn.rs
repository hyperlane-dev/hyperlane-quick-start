use super::*;

#[instrument_trace]
pub async fn create<P, F, Fut>(pid_path: P, server_hook: F)
where
    P: AsRef<str>,
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let args: Vec<String> = args().collect();
    debug!("Process create args{COLON_SPACE}{args:?}");
    let mut manager: ServerManager = ServerManager::new();
    manager
        .set_pid_file(pid_path.as_ref())
        .set_server_hook(server_hook);
    let is_daemon: bool = args.len() >= 3 && args[2].to_lowercase() == DAEMON_FLAG;
    let start_server = || async {
        if is_daemon {
            match manager.start_daemon().await {
                Ok(_) => info!("Server started in background successfully"),
                Err(error) => {
                    error!("Error starting server in background{COLON_SPACE}{error}")
                }
            };
        } else {
            info!("Server started successfully");
            manager.start().await;
        }
    };
    let stop_server = || async {
        match manager.stop().await {
            Ok(_) => info!("Server stopped successfully"),
            Err(error) => error!("Error stopping server{COLON_SPACE}{error}"),
        };
    };
    let restart_server = || async {
        stop_server().await;
        start_server().await;
    };
    if args.len() < 2 {
        warn!("No additional command-line parameters, default startup");
        start_server().await;
        return;
    }
    let command: String = args[1].to_lowercase();
    match command.as_str() {
        CMD_STOP => stop_server().await,
        CMD_RESTART => restart_server().await,
        _ => {
            error!("Invalid command{COLON_SPACE}{command}");
        }
    }
}
