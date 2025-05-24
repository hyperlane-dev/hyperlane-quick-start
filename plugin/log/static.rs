use super::*;

pub static LOG: Lazy<Log> = Lazy::new(|| {
    let mut log: Log = Log::default();
    log.set_limit_file_size(SERVER_LOG_SIZE)
        .set_path(SERVER_LOG_DIR.to_owned());
    log
});
