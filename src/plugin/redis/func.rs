use super::lazy::REDIS_CONNECT;
use redis::{self, Client, Commands, Connection, RedisResult};
use std::sync::RwLockWriteGuard;

pub async fn init() -> RedisResult<()> {
    let url: String = format!(
        "redis://{}:{}@{}:{}",
        crate::config::redis::USERNAME,
        crate::config::redis::PASSWORD,
        crate::config::redis::HOST,
        crate::config::redis::PORT
    );
    let client: Client = Client::open(url)?;
    let connect: Connection = client.get_connection()?;
    let mut redis_connect: RwLockWriteGuard<'_, Option<Connection>> =
        REDIS_CONNECT.write().unwrap();
    *redis_connect = Some(connect);
    Ok(())
}

pub fn get_connection_rw_lock() -> RwLockWriteGuard<'static, Option<Connection>> {
    REDIS_CONNECT.write().unwrap()
}

pub fn set_value(key: &str, value: &str) -> RedisResult<()> {
    let mut redis_connect_opt: RwLockWriteGuard<'_, Option<Connection>> = get_connection_rw_lock();
    if let Some(ref mut redis_connect) = *redis_connect_opt {
        let _: () = redis_connect.set(key, value)?;
    }
    Ok(())
}

pub fn get_value(key: &str) -> RedisResult<String> {
    let mut redis_connect_opt: RwLockWriteGuard<'_, Option<Connection>> = get_connection_rw_lock();
    if let Some(ref mut redis_connect) = *redis_connect_opt {
        let value: String = redis_connect.get(key)?;
        return Ok(value);
    }
    Ok(String::new())
}
