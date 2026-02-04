use super::*;

#[instrument_trace]
pub async fn print_route_matcher(server: &Server) {
    let route_matcher: RouteMatcher = server.get_route_matcher().await;
    for key in route_matcher.get_static_route().keys() {
        info!("Static route{COLON_SPACE}{key}");
    }
    for value in route_matcher.get_dynamic_route().values() {
        for (route_pattern, _) in value {
            info!("Dynamic route{COLON_SPACE}{route_pattern}");
        }
    }
    for value in route_matcher.get_regex_route().values() {
        for (route_pattern, _) in value {
            info!("Regex route{COLON_SPACE}{route_pattern}");
        }
    }
}

#[hyperlane(server: Server)]
#[instrument_trace]
pub async fn create_server() {
    init_log();
    if let Err(error) = init_env_config() {
        error!("{error}");
    }
    info!("Environment configuration loaded successfully");
    init_server_config(&server).await;
    init_db().await;
    init_cicd().await;
    match server.run().await {
        Ok(server_hook) => {
            let host_port: String = format!("{SERVER_HOST}{COLON}{SERVER_PORT}");
            print_route_matcher(&server).await;
            info!("Server listen in{COLON_SPACE}{host_port}");
            let shutdown: SharedAsyncTaskFactory<()> = server_hook.get_shutdown_hook().clone();
            set_shutdown(shutdown);
            server_hook.wait().await;
        }
        Err(server_error) => error!("Server run error{COLON_SPACE}{server_error}"),
    }
}
