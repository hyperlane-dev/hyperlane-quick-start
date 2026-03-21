use super::*;

impl BootstrapSyncInit for LoggerBootstrap {
    fn init() -> Self {
        let env_config: &EnvConfig = EnvPlugin::get_or_init();
        let mut file_logger: FileLogger = FileLogger::default();
        file_logger.set_path(env_config.get_server_log_dir());
        file_logger.set_limit_file_size(env_config.get_server_log_size());
        Logger::init(LOG_LEVEL_FILTER, file_logger);
        Self
    }
}
