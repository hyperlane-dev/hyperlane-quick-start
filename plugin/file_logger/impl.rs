use super::*;

/// Default implementation for FileLogger configuration.
impl Default for FileLogger {
    /// Creates default FileLogger configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Default FileLogger instance with default path and file size limit.
    #[inline(always)]
    fn default() -> Self {
        Self {
            path: DEFAULT_LOG_DIR.to_owned(),
            limit_file_size: DEFAULT_LOG_FILE_SIZE,
            trace_dir: TRACE_DIR.to_owned(),
            debug_dir: DEBUG_DIR.to_owned(),
            info_dir: INFO_DIR.to_owned(),
            warn_dir: WARN_DIR.to_owned(),
            error_dir: ERROR_DIR.to_owned(),
        }
    }
}

impl FileLogger {
    /// Creates new FileLogger configuration with specified parameters.
    ///
    /// # Arguments
    ///
    /// - `P` - The path for storing log files.
    /// - `usize` - The maximum file size limit in bytes.
    ///
    /// # Returns
    ///
    /// - `Self` - A new FileLogger instance with specified configuration.
    #[inline(always)]
    pub fn new<P: AsRef<str>>(path: P, limit_file_size: usize) -> Self {
        Self {
            path: path.as_ref().to_owned(),
            limit_file_size,
            ..Self::default()
        }
    }

    /// Checks if logging is enabled.
    ///
    /// # Returns
    ///
    /// - `bool` - True if logging is enabled.
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        self.get_limit_file_size() != DISABLE_LOG_FILE_SIZE
    }

    /// Checks if logging is disabled.
    ///
    /// # Returns
    ///
    /// - `bool` - True if logging is disabled.
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        !self.is_enable()
    }

    /// Writes log data synchronously to specified directory.
    ///
    /// # Arguments
    ///
    /// - `T` - The data to be logged.
    /// - `L` - The log formatting function.
    /// - `&str` - The subdirectory for log file.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self for method chaining.
    fn write_sync<T, L>(&self, data: T, func: L, dir: &str) -> &Self
    where
        T: AsRef<str>,
        L: Fn(T) -> String + Send + Sync,
    {
        if self.is_disable() {
            return self;
        }
        let out: String = func(data);
        let path: String = get_log_path(dir, self.get_path(), &self.get_limit_file_size());
        let _: Result<(), Error> = append_to_file(&path, out.as_bytes());
        self
    }

    /// Logs trace message synchronously.
    ///
    /// # Arguments
    ///
    /// - `T` - Trace data to be logged.
    /// - `L` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub fn trace<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: Fn(T) -> String + Send + Sync,
    {
        self.write_sync(data, func, self.get_trace_dir())
    }

    /// Logs debug message synchronously.
    ///
    /// # Arguments
    ///
    /// - `T` - Debug data to be logged.
    /// - `L` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub fn debug<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: Fn(T) -> String + Send + Sync,
    {
        self.write_sync(data, func, self.get_debug_dir())
    }

    /// Logs info message synchronously.
    ///
    /// # Arguments
    ///
    /// - `T` - Info data to be logged.
    /// - `L` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub fn info<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: Fn(T) -> String + Send + Sync,
    {
        self.write_sync(data, func, self.get_info_dir())
    }

    /// Logs warn message synchronously.
    ///
    /// # Arguments
    ///
    /// - `T` - Warn data to be logged.
    /// - `L` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub fn warn<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: Fn(T) -> String + Send + Sync,
    {
        self.write_sync(data, func, self.get_warn_dir())
    }

    /// Logs error message synchronously.
    ///
    /// # Arguments
    ///
    /// - `T` - Error data to be logged.
    /// - `L` - FileLogger formatting function.
    ///
    /// # Returns
    ///
    /// - `&Self` - Reference to self.
    pub fn error<T, L>(&self, data: T, func: L) -> &Self
    where
        T: AsRef<str>,
        L: Fn(T) -> String + Send + Sync,
    {
        self.write_sync(data, func, self.get_error_dir())
    }
}
