use super::*;

/// Global static storage for Redis connection caches, mapping instance names to their cached connections.
pub(crate) static REDIS_CONNECTIONS: OnceLock<RwLock<RedisConnectionMap>> = OnceLock::new();
