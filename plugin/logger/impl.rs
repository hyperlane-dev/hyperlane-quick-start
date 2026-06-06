use super::*;

/// Implementation of `GetOrInit` for `LoggerPlugin`, providing lazy initialization of the global file logger.
impl GetOrInit for LoggerPlugin {
    type Instance = RwLock<FileLogger>;

    /// Lazily initializes and returns a static reference to the global file logger.
    ///
    /// # Returns
    ///
    /// - `&'static RwLock<FileLogger>`: The static reference to the global file logger.
    fn get_or_init() -> &'static Self::Instance {
        FILE_LOGGER.get_or_init(|| RwLock::new(FileLogger::default()))
    }
}

/// Implementation of the `Log` trait for `Logger`, providing colored console output and file logging.
impl Log for Logger {
    /// Checks whether the given log metadata is enabled based on the current maximum log level.
    ///
    /// # Arguments
    ///
    /// - `&Metadata`: The log metadata to check.
    ///
    /// # Returns
    ///
    /// - `bool`: True if the metadata level is at or below the maximum level.
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= max_level()
    }

    /// Logs a record by outputting colored console output and writing to the file logger.
    ///
    /// # Arguments
    ///
    /// - `&Record`: The log record to output.
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let now_time: String = time();
        let level: Level = record.level();
        let args: &Arguments<'_> = record.args();
        let file: Option<&str> = record.file();
        let module_path: Option<&str> = record.module_path();
        let target: &str = record.target();
        let line: u32 = record.line().unwrap_or_default();
        let location: &str = file.unwrap_or(module_path.unwrap_or(target));
        let time_text: String = format!("{SPACE}{now_time}{SPACE}");
        let level_text: String = format!("{SPACE}{level}{SPACE}");
        let args_text: String = format!("{args}{SPACE}");
        let location_text: String = format!("{SPACE}{location}{COLON}{line}{SPACE}");
        let write_file_data: String = format!("{level}{location_text}{args}");
        let color: ColorType = match record.level() {
            Level::Trace => ColorType::Use(Color::Magenta),
            Level::Debug => ColorType::Use(Color::Cyan),
            Level::Info => ColorType::Use(Color::Green),
            Level::Warn => ColorType::Use(Color::Yellow),
            Level::Error => ColorType::Use(Color::Red),
        };
        let mut time_output_builder: ColorOutputBuilder<'_> = ColorOutputBuilder::new();
        let mut level_output_builder: ColorOutputBuilder<'_> = ColorOutputBuilder::new();
        let mut location_output_builder: ColorOutputBuilder<'_> = ColorOutputBuilder::new();
        let mut args_output_builder: ColorOutputBuilder<'_> = ColorOutputBuilder::new();
        let time_output: ColorOutput<'_> = time_output_builder
            .text(&time_text)
            .bold(true)
            .color(ColorType::Use(Color::White))
            .bg_color(ColorType::Use(Color::Black))
            .build();
        let level_output: ColorOutput<'_> = level_output_builder
            .text(&level_text)
            .bold(true)
            .color(ColorType::Use(Color::White))
            .bg_color(color)
            .build();
        let location_output: ColorOutput<'_> = location_output_builder
            .text(&location_text)
            .bold(true)
            .color(color)
            .build();
        let args_output: ColorOutput<'_> = args_output_builder
            .text(&args_text)
            .bold(true)
            .color(color)
            .endl(true)
            .build();
        ColorOutputListBuilder::new()
            .add(time_output)
            .add(level_output)
            .add(location_output)
            .add(args_output)
            .run();
        match record.metadata().level() {
            Level::Trace => Self::log_trace(&write_file_data),
            Level::Debug => Self::log_debug(&write_file_data),
            Level::Info => Self::log_info(&write_file_data),
            Level::Warn => Self::log_warn(&write_file_data),
            Level::Error => Self::log_error(&write_file_data),
        }
    }

    /// Flushes the standard output and standard error streams.
    fn flush(&self) {
        Server::flush_stdout_and_stderr();
    }
}

/// Implementation of initialization and logging methods for `Logger`.
impl Logger {
    /// Acquires a read lock on the global file logger.
    ///
    /// # Returns
    ///
    /// - `RwLockReadGuard<'static, FileLogger>`: The read guard for the file logger.
    fn read() -> RwLockReadGuard<'static, FileLogger> {
        LoggerPlugin::get_or_init().try_read().unwrap()
    }

    /// Acquires a write lock on the global file logger.
    ///
    /// # Returns
    ///
    /// - `RwLockWriteGuard<'static, FileLogger>`: The write guard for the file logger.
    fn write() -> RwLockWriteGuard<'static, FileLogger> {
        LoggerPlugin::get_or_init().try_write().unwrap()
    }

    /// Initializes the global logger with the specified log level and file logger configuration.
    ///
    /// # Arguments
    ///
    /// - `LevelFilter`: The maximum log level to enable.
    /// - `FileLogger`: The file logger configuration for writing logs to files.
    pub fn init(level: LevelFilter, file_logger: FileLogger) {
        set_logger(&LOGGER).unwrap();
        set_max_level(level);
        *Self::write() = file_logger;
    }

    /// Logs a trace-level message to the file logger.
    ///
    /// # Arguments
    ///
    /// - `T`: The data to log, which must implement `AsRef<str>`.
    pub fn log_trace<T>(data: T)
    where
        T: AsRef<str>,
    {
        Self::read().trace(data, log_handler);
    }

    /// Logs a debug-level message to the file logger.
    ///
    /// # Arguments
    ///
    /// - `T`: The data to log, which must implement `AsRef<str>`.
    #[instrument_trace]
    pub fn log_debug<T>(data: T)
    where
        T: AsRef<str>,
    {
        Self::read().debug(data, log_handler);
    }

    /// Logs an info-level message to the file logger.
    ///
    /// # Arguments
    ///
    /// - `T`: The data to log, which must implement `AsRef<str>`.
    #[instrument_trace]
    pub fn log_info<T>(data: T)
    where
        T: AsRef<str>,
    {
        Self::read().info(data, log_handler);
    }

    /// Logs a warning-level message to the file logger.
    ///
    /// # Arguments
    ///
    /// - `T`: The data to log, which must implement `AsRef<str>`.
    #[instrument_trace]
    pub fn log_warn<T>(data: T)
    where
        T: AsRef<str>,
    {
        Self::read().warn(data, log_handler);
    }

    /// Logs an error-level message to the file logger.
    ///
    /// # Arguments
    ///
    /// - `T`: The data to log, which must implement `AsRef<str>`.
    #[instrument_trace]
    pub fn log_error<T>(data: T)
    where
        T: AsRef<str>,
    {
        Self::read().error(data, log_handler);
    }
}
