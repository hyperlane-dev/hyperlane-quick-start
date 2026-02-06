use super::*;

impl RedisService {
    #[instrument_trace]
    pub async fn create_redis_record(record: RedisRecord) -> Result<(), String> {
        let conn_arc: ArcRwLock<Connection> =
            get_redis_connection(DEFAULT_REDIS_INSTANCE_NAME).await?;
        let dao: RedisRecordDao = RedisRecordDao {
            key: record.get_key().clone(),
            value: record.get_value().clone(),
        };
        let mut conn: RwLockWriteGuard<'_, Connection> = conn_arc.write().await;
        let _: () = Commands::set(&mut *conn, dao.get_key(), dao.get_value())
            .map_err(|error: RedisError| error.to_string())?;
        Ok(())
    }

    #[instrument_trace]
    pub async fn get_all_redis_records() -> Result<Vec<RedisRecord>, String> {
        let conn_arc: ArcRwLock<Connection> =
            get_redis_connection(DEFAULT_REDIS_INSTANCE_NAME).await?;
        let mut conn: RwLockWriteGuard<'_, Connection> = conn_arc.write().await;
        let all_keys: Vec<String> = cmd("KEYS")
            .arg("*")
            .query(&mut *conn)
            .map_err(|error: RedisError| error.to_string())?;
        if all_keys.is_empty() {
            return Ok(Vec::new());
        }
        let values: Vec<String> =
            Commands::mget(&mut *conn, &all_keys).map_err(|error: RedisError| error.to_string())?;
        let records: Vec<RedisRecord> = all_keys
            .into_iter()
            .zip(values)
            .map(|(k, v): (String, String)| {
                let mut record = RedisRecord::default();
                record.set_key(k).set_value(v);
                record
            })
            .collect();
        Ok(records)
    }

    #[instrument_trace]
    pub async fn update_redis_record(record: RedisRecord) -> Result<(), String> {
        let conn_arc: ArcRwLock<Connection> =
            get_redis_connection(DEFAULT_REDIS_INSTANCE_NAME).await?;
        let mut conn: RwLockWriteGuard<'_, Connection> = conn_arc.write().await;
        let _: () = Commands::set(&mut *conn, record.get_key(), record.get_value())
            .map_err(|error: RedisError| error.to_string())?;
        Ok(())
    }

    #[instrument_trace]
    pub async fn delete_redis_record(key: &str) -> Result<(), String> {
        let conn_arc: ArcRwLock<Connection> =
            get_redis_connection(DEFAULT_REDIS_INSTANCE_NAME).await?;
        let mut conn: RwLockWriteGuard<'_, Connection> = conn_arc.write().await;
        let _: () =
            Commands::del(&mut *conn, key).map_err(|error: RedisError| error.to_string())?;
        Ok(())
    }
}
