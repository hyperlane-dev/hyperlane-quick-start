use super::*;

pub static REDIS_CONNECTIONS: OnceLock<RwLock<RedisConnectionMap>> = OnceLock::new();
