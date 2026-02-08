use super::*;

impl BootstrapSyncInit for LoggerBootstrap {
    fn init() -> Self {
        let mut file_logger: FileLogger = FileLogger::default();
        file_logger.set_path(SERVER_LOG_DIR);
        file_logger.set_limit_file_size(SERVER_LOG_SIZE);
        Logger::init(LOG_LEVEL_FILTER, file_logger);
        Self
    }
}
