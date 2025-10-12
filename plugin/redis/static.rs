use super::*;

pub static REDIS_DB: Lazy<Result<Arc<Connection>, String>> =
    Lazy::new(|| block_on(async { connection_redis_db().await }));
