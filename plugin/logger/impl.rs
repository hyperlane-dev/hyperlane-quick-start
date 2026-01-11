use super::*;

impl Logger {
    pub fn log_trace<T>(data: T)
    where
        T: AsRef<str>,
    {
        FILE_LOGGER.trace(data, log_handler);
    }

    pub fn log_debug<T>(data: T)
    where
        T: AsRef<str>,
    {
        FILE_LOGGER.debug(data, log_handler);
    }

    pub fn log_info<T>(data: T)
    where
        T: AsRef<str>,
    {
        FILE_LOGGER.info(data, log_handler);
    }

    pub fn log_warn<T>(data: T)
    where
        T: AsRef<str>,
    {
        FILE_LOGGER.warn(data, log_handler);
    }

    pub fn log_error<T>(data: T)
    where
        T: AsRef<str>,
    {
        FILE_LOGGER.error(data, log_handler);
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= LOG_LEVEL_FILTER
    }

    fn log(&self, record: &Record) {
        let time_text: String = format!("{SPACE}{}{SPACE}", time());
        let level_text: String = format!("{SPACE}{}{SPACE}", record.level());
        let args_text: String = format!("{SPACE}{}{SPACE}", record.args());
        let write_file_data: String = format!("{} {}", record.level(), record.args());
        match record.metadata().level() {
            Level::Trace => Self::log_trace(&write_file_data),
            Level::Debug => Self::log_debug(&write_file_data),
            Level::Info => Self::log_info(&write_file_data),
            Level::Warn => Self::log_warn(&write_file_data),
            Level::Error => Self::log_error(&write_file_data),
        }
        if !self.enabled(record.metadata()) {
            return;
        }
        let mut time_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let mut level_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let mut args_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let time_output: Output<'_> = time_output_builder
            .text(&time_text)
            .bold(true)
            .color(ColorType::Use(Color::White))
            .bg_color(ColorType::Use(Color::Black))
            .build();
        let level_output: Output<'_> = level_output_builder
            .text(&level_text)
            .bold(true)
            .color(ColorType::Use(Color::White))
            .bg_color(match record.level() {
                Level::Trace => ColorType::Use(Color::Magenta),
                Level::Debug => ColorType::Use(Color::Cyan),
                Level::Info => ColorType::Use(Color::Green),
                Level::Warn => ColorType::Use(Color::Yellow),
                Level::Error => ColorType::Use(Color::Red),
            })
            .build();
        let args_output: Output<'_> = args_output_builder
            .text(&args_text)
            .bold(true)
            .endl(true)
            .color(match record.level() {
                Level::Trace => ColorType::Use(Color::Magenta),
                Level::Debug => ColorType::Use(Color::Cyan),
                Level::Info => ColorType::Use(Color::Green),
                Level::Warn => ColorType::Use(Color::Yellow),
                Level::Error => ColorType::Use(Color::Red),
            })
            .build();
        OutputListBuilder::new()
            .add(time_output)
            .add(level_output)
            .add(args_output)
            .run();
    }

    fn flush(&self) {
        Server::flush_stdout_and_stderr();
    }
}

impl Logger {
    pub fn init(level: LevelFilter) {
        set_logger(&LOGGER).unwrap();
        set_max_level(level);
    }

    pub fn trace<T>(data: T)
    where
        T: AsRef<str>,
    {
        trace!("{}", data.as_ref());
    }

    pub fn debug<T>(data: T)
    where
        T: AsRef<str>,
    {
        debug!("{}", data.as_ref());
    }

    pub fn info<T>(data: T)
    where
        T: AsRef<str>,
    {
        info!("{}", data.as_ref());
    }

    pub fn warn<T>(data: T)
    where
        T: AsRef<str>,
    {
        warn!("{}", data.as_ref());
    }

    pub fn error<T>(data: T)
    where
        T: AsRef<str>,
    {
        error!("{}", data.as_ref());
    }
}
