use super::*;

impl RedisService {
    pub async fn create_redis_record(record: RedisRecord) -> Result<(), String> {
        let conn_arc: Arc<Connection> = get_redis_connection().await?;
        let dao: RedisRecordDao = RedisRecordDao {
            key: record.get_key().clone(),
            value: record.get_value().clone(),
        };
        let mut conn: Connection = Arc::try_unwrap(conn_arc)
            .map_err(|_| "Failed to get exclusive access to connection")?;
        let _: () = Commands::set(&mut conn, &dao.key, &dao.value)
            .map_err(|error: RedisError| error.to_string())?;
        Ok(())
    }

    pub async fn get_all_redis_records(keys: Vec<String>) -> Result<Vec<RedisRecord>, String> {
        let conn_arc: Arc<Connection> = get_redis_connection().await?;
        let mut conn: Connection = Arc::try_unwrap(conn_arc)
            .map_err(|_| "Failed to get exclusive access to connection")?;
        let values: Vec<String> =
            Commands::mget(&mut conn, &keys).map_err(|error: RedisError| error.to_string())?;
        let records: Vec<RedisRecord> = keys
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

    pub async fn update_redis_record(record: RedisRecord) -> Result<(), String> {
        let conn_arc: Arc<Connection> = get_redis_connection().await?;
        let mut conn: Connection = Arc::try_unwrap(conn_arc)
            .map_err(|_| "Failed to get exclusive access to connection")?;
        let _: () = Commands::set(&mut conn, record.get_key(), record.get_value())
            .map_err(|error: RedisError| error.to_string())?;
        Ok(())
    }

    pub async fn delete_redis_record(key: &str) -> Result<(), String> {
        let conn_arc: Arc<Connection> = get_redis_connection().await?;
        let mut conn: Connection = Arc::try_unwrap(conn_arc)
            .map_err(|_| "Failed to get exclusive access to connection")?;
        let _: () = Commands::del(&mut conn, key).map_err(|error: RedisError| error.to_string())?;
        Ok(())
    }
}
