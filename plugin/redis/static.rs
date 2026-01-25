use super::*;

pub static REDIS_CONNECTIONS: Lazy<RwLock<RedisConnectionMap>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
