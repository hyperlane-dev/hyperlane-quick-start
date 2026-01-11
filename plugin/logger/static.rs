use super::*;

pub(super) static LOGGER: Logger = Logger;
pub(super) static FILE_LOGGER: Lazy<FileLogger> = Lazy::new(|| {
    let mut file_logger: FileLogger = FileLogger::default();
    file_logger.path(SERVER_LOG_DIR);
    file_logger.limit_file_size(SERVER_LOG_SIZE);
    file_logger
});
