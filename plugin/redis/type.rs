use super::*;

pub type RedisConnectionResult = Result<Arc<RwLock<Connection>>, String>;

pub type RedisConnectionMap = HashMap<String, ConnectionCache<Arc<RwLock<Connection>>>>;
