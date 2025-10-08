use super::*;

pub static REDIS_DB: Lazy<Arc<Connection>> =
    Lazy::new(|| block_on(async { connection_redis_db().await }));
