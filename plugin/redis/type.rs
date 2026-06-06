use super::*;

/// Type alias for a HashMap mapping Redis instance names to their cached connections.
pub type RedisConnectionMap = HashMap<String, ConnectionCache<ArcRwLock<Connection>>>;
