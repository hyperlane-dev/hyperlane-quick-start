use super::*;

/// Implementation of methods for `RedisService`.
impl RedisService {
    /// Creates a new Redis record by setting a key-value pair.
    ///
    /// # Arguments
    ///
    /// - `RedisRecord`: The record containing the key and value to store.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the Redis operation fails.
    #[instrument_trace]
    pub async fn create_redis_record(record: RedisRecord) -> Result<(), String> {
        let conn_arc: ArcRwLock<Connection> =
            RedisPlugin::get_connection(DEFAULT_REDIS_INSTANCE_NAME, None).await?;
        let dao: RedisRecordDao = RedisRecordDao {
            key: record.get_key().clone(),
            value: record.get_value().clone(),
        };
        let mut conn: RwLockWriteGuard<'_, Connection> = conn_arc.write().await;
        let _: () = Commands::set(&mut *conn, dao.get_key(), dao.get_value())
            .map_err(|error: RedisError| error.to_string())?;
        Ok(())
    }

    /// Retrieves all Redis records by fetching all keys and their corresponding values.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<RedisRecord>, String>`: The list of all Redis records, or an error if the operation fails.
    #[instrument_trace]
    pub async fn get_all_redis_records() -> Result<Vec<RedisRecord>, String> {
        let conn_arc: ArcRwLock<Connection> =
            RedisPlugin::get_connection(DEFAULT_REDIS_INSTANCE_NAME, None).await?;
        let mut conn: RwLockWriteGuard<'_, Connection> = conn_arc.write().await;
        let all_keys: Vec<String> = cmd(REDIS_KEYS_COMMAND)
            .arg(REDIS_KEYS_PATTERN_ALL)
            .query(&mut *conn)
            .map_err(|error: RedisError| error.to_string())?;
        if all_keys.is_empty() {
            return Ok(vec![]);
        }
        let values: Vec<String> =
            Commands::mget(&mut *conn, &all_keys).map_err(|error: RedisError| error.to_string())?;
        let records: Vec<RedisRecord> = all_keys
            .into_iter()
            .zip(values)
            .map(|(k, v): (String, String)| {
                let mut record: RedisRecord = RedisRecord::default();
                record.set_key(k).set_value(v);
                record
            })
            .collect();
        Ok(records)
    }

    /// Updates an existing Redis record by setting the key to a new value.
    ///
    /// # Arguments
    ///
    /// - `RedisRecord`: The record containing the key and updated value.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the Redis operation fails.
    #[instrument_trace]
    pub async fn update_redis_record(record: RedisRecord) -> Result<(), String> {
        let conn_arc: ArcRwLock<Connection> =
            RedisPlugin::get_connection(DEFAULT_REDIS_INSTANCE_NAME, None).await?;
        let mut conn: RwLockWriteGuard<'_, Connection> = conn_arc.write().await;
        let _: () = Commands::set(&mut *conn, record.get_key(), record.get_value())
            .map_err(|error: RedisError| error.to_string())?;
        Ok(())
    }

    /// Deletes a Redis record by key.
    ///
    /// # Arguments
    ///
    /// - `&str`: The key of the record to delete.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the Redis operation fails.
    #[instrument_trace]
    pub async fn delete_redis_record(key: &str) -> Result<(), String> {
        let conn_arc: ArcRwLock<Connection> =
            RedisPlugin::get_connection(DEFAULT_REDIS_INSTANCE_NAME, None).await?;
        let mut conn: RwLockWriteGuard<'_, Connection> = conn_arc.write().await;
        let _: () =
            Commands::del(&mut *conn, key).map_err(|error: RedisError| error.to_string())?;
        Ok(())
    }
}
