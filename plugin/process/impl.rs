use super::*;

/// Implementation of process lifecycle management for `ProcessPlugin`.
impl ProcessPlugin {
    /// Creates and manages the server process based on command-line arguments.
    ///
    /// Supports `stop`, `restart` commands and a `-d` flag for daemon mode.
    /// If no command is provided, the server starts in foreground mode by default.
    ///
    /// # Arguments
    ///
    /// - `P` - The PID file path for process management.
    /// - `F` - The server hook function that returns a future to run the server.
    ///
    /// # Panics
    ///
    /// This function does not explicitly panic, but the underlying `ServerManager` operations may panic on critical failures.
    #[instrument_trace]
    pub async fn create<P, F, Fut>(pid_path: P, server_hook: F)
    where
        P: AsRef<str>,
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let args: Vec<String> = args().collect();
        debug!("Process create args {args:?}");
        trace!("Pid file path: {}", pid_path.as_ref());
        let mut manager: ServerManager = ServerManager::new();
        manager
            .set_pid_file(pid_path.as_ref().to_string())
            .set_server_hook(Arc::new(move || Box::pin(server_hook())));
        let is_daemon: bool = args.len() >= 3 && args[2].to_lowercase() == DAEMON_FLAG;
        if args.len() < 2 {
            warn!("No additional command-line parameters, default startup");
            Self::start_managed_server(&mut manager, is_daemon).await;
            return;
        }
        let command: String = args[1].to_lowercase();
        match command.as_str() {
            CMD_STOP => Self::stop_managed_server(&mut manager).await,
            CMD_RESTART => {
                Self::stop_managed_server(&mut manager).await;
                Self::start_managed_server(&mut manager, is_daemon).await;
            }
            _ => {
                error!("Invalid command {command}");
            }
        }
    }

    /// Starts the managed server, honoring daemon mode.
    ///
    /// # Arguments
    ///
    /// - `&mut ServerManager` - The server manager to start.
    /// - `bool` - Whether the server is started as a background daemon.
    ///
    /// # Returns
    ///
    /// - `()` - The future resolves once the start attempt has finished.
    async fn start_managed_server(manager: &mut ServerManager, is_daemon: bool) {
        if is_daemon {
            match manager.start_daemon().await {
                Ok(_) => info!("Server started in background successfully"),
                Err(error) => {
                    error!("Error starting server in background {error}")
                }
            };
        } else {
            info!("Server started successfully");
            manager.start().await;
        }
    }

    /// Stops the managed server.
    ///
    /// # Arguments
    ///
    /// - `&mut ServerManager` - The server manager to stop.
    ///
    /// # Returns
    ///
    /// - `()` - The future resolves once the stop attempt has finished.
    async fn stop_managed_server(manager: &mut ServerManager) {
        match manager.stop().await {
            Ok(_) => info!("Server stopped successfully"),
            Err(error) => error!("Error stopping server {error}"),
        };
    }
}
