/// Default directory path for storing log files.
pub(crate) const DEFAULT_LOG_DIR: &str = "./logs";

/// File extension for log files.
pub(crate) const LOG_EXTENSION: &str = "log";

/// Default starting index number for log files.
pub(crate) const DEFAULT_LOG_FILE_START_IDX: usize = 1;

/// Default maximum size limit for log files in bytes.
pub(crate) const DEFAULT_LOG_FILE_SIZE: usize = 1_024_000_000;

/// Special value indicating no size limit for log files.
pub(crate) const DISABLE_LOG_FILE_SIZE: usize = 0;

/// Subdirectory name for trace logs.
pub(crate) const TRACE_DIR: &str = "trace";

/// Subdirectory name for debug logs.
pub(crate) const DEBUG_DIR: &str = "debug";

/// Subdirectory name for info logs.
pub(crate) const INFO_DIR: &str = "info";

/// Subdirectory name for warn logs.
pub(crate) const WARN_DIR: &str = "warn";

/// Subdirectory name for error logs.
pub(crate) const ERROR_DIR: &str = "error";
