use super::*;

/// Implementation of `BootstrapSyncInit` for `LoggerBootstrap`, configuring and initializing the logger on startup.
impl BootstrapSyncInit for LoggerBootstrap {
    /// Initializes the logger bootstrap by loading environment configuration and configuring the file logger.
    ///
    /// # Returns
    ///
    /// - `Self` - The initialized `LoggerBootstrap` instance.
    fn init() -> Self {
        let env_config: &EnvConfig = EnvPlugin::get_or_init();
        // `color_log` and `hyperlane_plugin::logger` both re-export a
        // `FileLogger`, so `super::*` leaves the name ambiguous (E0659).
        // `Logger::init` takes the plugin crate's own type, so that one is
        // named at the place of use, without a rename in the import.
        let mut file_logger: hyperlane_plugin::logger::FileLogger =
            hyperlane_plugin::logger::FileLogger::default();
        file_logger.set_path(env_config.get_server_log_dir().to_string());
        file_logger.set_limit_file_size(env_config.get_server_log_size());
        Logger::init(LOG_LEVEL_FILTER, file_logger);
        Self
    }
}
