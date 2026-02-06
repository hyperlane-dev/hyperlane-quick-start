use super::*;

pub type RedisConnectionMap = HashMap<String, ConnectionCache<ArcRwLock<Connection>>>;
