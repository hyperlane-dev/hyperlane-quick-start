use super::*;

impl Default for RedisAutoCreation {
    #[instrument_trace]
    fn default() -> Self {
        let env: &'static EnvConfig = EnvPlugin::get_or_init_global_env_config();
        if let Some(instance) = env.get_default_redis_instance() {
            Self::new(instance.clone())
        } else {
            let default_instance: RedisInstanceConfig = RedisInstanceConfig::default();
            Self::new(default_instance)
        }
    }
}

impl RedisAutoCreation {
    #[instrument_trace]
    async fn create_mutable_connection(&self) -> Result<Connection, AutoCreationError> {
        let db_url: String = self.instance.get_connection_url();
        let client: Client = Client::open(db_url).map_err(|error: RedisError| {
            let error_msg: String = error.to_string();
            if error_msg.contains("authentication failed") || error_msg.contains("NOAUTH") {
                AutoCreationError::InsufficientPermissions(format!(
                    "Redis authentication failed {error_msg}"
                ))
            } else if error_msg.contains("Connection refused") || error_msg.contains("timeout") {
                AutoCreationError::ConnectionFailed(format!(
                    "Cannot connect to Redis server {error_msg}"
                ))
            } else {
                AutoCreationError::DatabaseError(format!("Redis connection error {error_msg}"))
            }
        })?;
        let timeout_duration: Duration = DatabasePlugin::get_connection_timeout_duration();
        let timeout_seconds: u64 = timeout_duration.as_secs();
        let connection_task: tokio::task::JoinHandle<Result<Connection, RedisError>> =
            tokio::task::spawn_blocking(move || client.get_connection());
        let connection: Connection = match timeout(timeout_duration, connection_task).await {
            Ok(join_result) => match join_result {
                Ok(result) => result.map_err(|error: RedisError| {
                    let error_msg: String = error.to_string();
                    if error_msg.contains("authentication failed") || error_msg.contains("NOAUTH") {
                        AutoCreationError::InsufficientPermissions(format!(
                            "Redis authentication failed {error_msg}"
                        ))
                    } else if error_msg.contains("Connection refused")
                        || error_msg.contains("timeout")
                    {
                        AutoCreationError::ConnectionFailed(format!(
                            "Cannot connect to Redis server {error_msg}"
                        ))
                    } else {
                        AutoCreationError::DatabaseError(format!(
                            "Redis connection error {error_msg}"
                        ))
                    }
                })?,
                Err(_) => {
                    return Err(AutoCreationError::ConnectionFailed(
                        "Redis connection task failed".to_string(),
                    ));
                }
            },
            Err(_) => {
                return Err(AutoCreationError::Timeout(format!(
                    "Redis connection timeout after {timeout_seconds} seconds"
                )));
            }
        };
        Ok(connection)
    }

    #[instrument_trace]
    async fn validate_redis_server(&self) -> Result<(), AutoCreationError> {
        let mut conn: Connection = self.create_mutable_connection().await?;
        let pong: String = redis::cmd("PING")
            .query(&mut conn)
            .map_err(|error: RedisError| {
                AutoCreationError::ConnectionFailed(format!("Redis PING failed {error}"))
            })?;
        if pong != "PONG" {
            return Err(AutoCreationError::ConnectionFailed(
                "Redis PING returned unexpected response".to_string(),
            ));
        }
        let info: String =
            redis::cmd("INFO")
                .arg("server")
                .query(&mut conn)
                .map_err(|error: RedisError| {
                    AutoCreationError::DatabaseError(format!(
                        "Failed to get Redis server info {error}"
                    ))
                })?;
        if info.contains("redis_version:") {
            AutoCreationLogger::log_connection_verification(
                database::PluginType::Redis,
                self.instance.get_name().as_str(),
                true,
                None,
            )
            .await;
        }
        Ok(())
    }

    #[instrument_trace]
    async fn setup_redis_namespace(&self) -> Result<Vec<String>, AutoCreationError> {
        let mut setup_operations: Vec<String> = Vec::new();
        let mut conn: Connection = self.create_mutable_connection().await?;
        let app_key: String = format!("{}:initialized", self.instance.get_name());
        let exists: i32 = redis::cmd("EXISTS")
            .arg(&app_key)
            .query(&mut conn)
            .map_err(|error: RedisError| {
                AutoCreationError::DatabaseError(format!(
                    "Failed to check Redis key existence {error}"
                ))
            })?;
        if exists == 0 {
            let _: () = redis::cmd("SET")
                .arg(&app_key)
                .arg("true")
                .query(&mut conn)
                .map_err(|error: RedisError| {
                    AutoCreationError::DatabaseError(format!(
                        "Failed to set Redis initialization key {error}"
                    ))
                })?;
            setup_operations.push(app_key.clone());
            let config_key: String = format!("{}:config:version", self.instance.get_name());
            let _: () = redis::cmd("SET")
                .arg(&config_key)
                .arg("1.0.0")
                .query(&mut conn)
                .map_err(|error: RedisError| {
                    AutoCreationError::DatabaseError(format!(
                        "Failed to set Redis config key {error}"
                    ))
                })?;
            setup_operations.push(config_key);
        }
        Ok(setup_operations)
    }
}

impl DatabaseAutoCreation for RedisAutoCreation {
    #[instrument_trace]
    async fn create_database_if_not_exists(&self) -> Result<bool, AutoCreationError> {
        self.validate_redis_server().await?;
        AutoCreationLogger::log_database_exists(
            self.instance.get_name().as_str(),
            database::PluginType::Redis,
        )
        .await;
        Ok(false)
    }

    #[instrument_trace]
    async fn create_tables_if_not_exist(&self) -> Result<Vec<String>, AutoCreationError> {
        let setup_operations: Vec<String> = self.setup_redis_namespace().await?;
        if !setup_operations.is_empty() {
            AutoCreationLogger::log_tables_created(
                &setup_operations,
                self.instance.get_name().as_str(),
                database::PluginType::Redis,
            )
            .await;
        } else {
            AutoCreationLogger::log_tables_created(
                &[],
                self.instance.get_name().as_str(),
                database::PluginType::Redis,
            )
            .await;
        }
        Ok(setup_operations)
    }

    #[instrument_trace]
    async fn verify_connection(&self) -> Result<(), AutoCreationError> {
        match self.validate_redis_server().await {
            Ok(_) => {
                AutoCreationLogger::log_connection_verification(
                    database::PluginType::Redis,
                    self.instance.get_name().as_str(),
                    true,
                    None,
                )
                .await;
                Ok(())
            }
            Err(error) => {
                AutoCreationLogger::log_connection_verification(
                    database::PluginType::Redis,
                    self.instance.get_name().as_str(),
                    false,
                    Some(&error.to_string()),
                )
                .await;
                Err(error)
            }
        }
    }
}
