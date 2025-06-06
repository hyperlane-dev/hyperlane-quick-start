use super::*;

pub static LOG: Lazy<Log> = Lazy::new(|| {
    let mut log: Log = Log::default();
    log.path(SERVER_LOG_DIR);
    log.limit_file_size(SERVER_LOG_SIZE);
    log
});
