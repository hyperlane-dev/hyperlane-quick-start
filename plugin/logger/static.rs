use super::*;

pub(super) static LOGGER: Logger = Logger;

pub(super) static FILE_LOGGER: Lazy<RwLock<FileLogger>> =
    Lazy::new(|| RwLock::new(FileLogger::default()));
