use super::*;

pub(super) fn get_or_init_file_logger() -> &'static RwLock<FileLogger> {
    FILE_LOGGER.get_or_init(|| RwLock::new(FileLogger::default()))
}
