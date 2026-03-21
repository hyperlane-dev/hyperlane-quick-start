use super::*;

impl ConfigBootstrap {
    pub fn get_server_port() -> u16 {
        EnvPlugin::get_or_init().get_server_port()
    }

    pub fn get_server_host() -> &'static str {
        &EnvPlugin::get_or_init().get_server_host()
    }

    pub fn get_server_buffer() -> usize {
        EnvPlugin::get_or_init().get_server_buffer()
    }

    pub fn get_server_log_size() -> usize {
        EnvPlugin::get_or_init().get_server_log_size()
    }

    pub fn get_server_log_dir() -> &'static str {
        &EnvPlugin::get_or_init().get_server_log_dir()
    }

    pub fn get_server_inner_print() -> bool {
        EnvPlugin::get_or_init().get_server_inner_print()
    }

    pub fn get_server_inner_log() -> bool {
        EnvPlugin::get_or_init().get_server_inner_log()
    }

    pub fn get_server_nodelay() -> Option<bool> {
        EnvPlugin::get_or_init().get_server_nodelay()
    }

    pub fn get_server_tti() -> Option<u32> {
        EnvPlugin::get_or_init().get_server_tti()
    }

    pub fn get_server_pid_file_path() -> &'static str {
        &EnvPlugin::get_or_init().get_server_pid_file_path()
    }

    pub fn get_server_request_http_read_timeout_ms() -> u64 {
        EnvPlugin::get_or_init().get_server_request_http_read_timeout_ms()
    }

    pub fn get_server_request_max_body_size() -> usize {
        EnvPlugin::get_or_init().get_server_request_max_body_size()
    }
}

impl BootstrapAsyncInit for ConfigBootstrap {
    #[hyperlane(server_config: ServerConfig)]
    async fn init() -> Self {
        let mut request_config: RequestConfig = RequestConfig::default();
        request_config
            .set_max_body_size(Self::get_server_request_max_body_size())
            .set_read_timeout_ms(Self::get_server_request_http_read_timeout_ms());
        server_config.set_address(Server::format_bind_address(
            Self::get_server_host(),
            Self::get_server_port(),
        ));
        server_config.set_ttl(Self::get_server_tti());
        server_config.set_nodelay(Self::get_server_nodelay());
        debug!("Server config {server_config:?}");
        info!("Server initialization successful");
        Self {
            server_config,
            request_config,
        }
    }
}
