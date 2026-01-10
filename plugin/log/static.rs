use super::*;

pub(super) static LOGGER: Logger = Logger;

pub(super) static LOG: Lazy<ServerLog> = Lazy::new(|| {
    let mut log: ServerLog = ServerLog::default();
    log.path(SERVER_LOG_DIR);
    log.limit_file_size(SERVER_LOG_SIZE);
    log
});
