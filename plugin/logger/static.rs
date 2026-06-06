use super::*;

/// Global static instance of the `Logger` used as the standard log handler.
pub(super) static LOGGER: Logger = Logger;
/// Global static storage for the file logger, protected by a read-write lock.
pub(super) static FILE_LOGGER: OnceLock<RwLock<FileLogger>> = OnceLock::new();
