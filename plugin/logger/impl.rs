use super::*;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= LOG_LEVEL_FILTER
    }

    fn log(&self, record: &Record) {
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
        let args_text: String = format!("{SPACE}{args}{SPACE}");
        let location_text: String = format!("{SPACE}{location}{COLON}{line}{SPACE}");
        let write_file_data: String = format!("{level}{location_text}{args}");
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
        let color: ColorType = match record.level() {
            Level::Trace => ColorType::Use(Color::Magenta),
            Level::Debug => ColorType::Use(Color::Cyan),
            Level::Info => ColorType::Use(Color::Green),
            Level::Warn => ColorType::Use(Color::Yellow),
            Level::Error => ColorType::Use(Color::Red),
        };
        let mut time_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let mut level_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let mut location_output_builder: OutputBuilder<'_> = OutputBuilder::new();
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
            .bg_color(color)
            .build();
        let location_output: Output<'_> = location_output_builder
            .text(&location_text)
            .bold(true)
            .color(color)
            .build();
        let args_output: Output<'_> = args_output_builder
            .text(&args_text)
            .bold(true)
            .color(color)
            .endl(true)
            .build();
        OutputListBuilder::new()
            .add(time_output)
            .add(level_output)
            .add(location_output)
            .add(args_output)
            .run();
    }

    fn flush(&self) {
        Server::flush_stdout_and_stderr();
    }
}

impl Logger {
    pub fn init(level: LevelFilter) {
        trace!("Logger init");
        set_logger(&LOGGER).unwrap();
        set_max_level(level);
    }

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
