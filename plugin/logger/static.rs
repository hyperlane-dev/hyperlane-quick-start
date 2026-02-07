use super::*;

pub(super) static LOGGER: Logger = Logger;
pub(super) static FILE_LOGGER: OnceLock<RwLock<FileLogger>> = OnceLock::new();
