use super::*;

pub type RedisConnectionResult = Result<Arc<Connection>, String>;

pub type RedisConnectionMap = HashMap<String, RedisConnectionResult>;
