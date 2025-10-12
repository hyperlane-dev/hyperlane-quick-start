use super::*;

impl RedisAutoCreation {
    pub fn new() -> Self {
        Self {
            env: get_global_env_config(),
        }
    }

    async fn create_mutable_connection(&self) -> Result<Connection, AutoCreationError> {
        let db_url: String = format!(
            "redis://{}:{}@{}:{}",
            self.env.redis_username,
            self.env.redis_password,
            self.env.redis_host,
            self.env.redis_port
        );
        let client: Client = Client::open(db_url).map_err(|error: redis::RedisError| {
            let error_msg: String = error.to_string();
            if error_msg.contains("authentication failed") || error_msg.contains("NOAUTH") {
                AutoCreationError::InsufficientPermissions(format!(
                    "Redis authentication failed: {error_msg}"
                ))
            } else if error_msg.contains("Connection refused") || error_msg.contains("timeout") {
                AutoCreationError::ConnectionFailed(format!(
                    "Cannot connect to Redis server: {error_msg}"
                ))
            } else {
                AutoCreationError::DatabaseError(format!("Redis connection error: {error_msg}"))
            }
        })?;
        let connection: Connection =
            client
                .get_connection()
                .map_err(|error: redis::RedisError| {
                    let error_msg: String = error.to_string();
                    if error_msg.contains("authentication failed") || error_msg.contains("NOAUTH") {
                        AutoCreationError::InsufficientPermissions(format!(
                            "Redis authentication failed: {error_msg}"
                        ))
                    } else if error_msg.contains("Connection refused")
                        || error_msg.contains("timeout")
                    {
                        AutoCreationError::ConnectionFailed(format!(
                            "Cannot connect to Redis server: {error_msg}"
                        ))
                    } else {
                        AutoCreationError::DatabaseError(format!(
                            "Redis connection error: {error_msg}"
                        ))
                    }
                })?;
        Ok(connection)
    }

    async fn validate_redis_server(&self) -> Result<(), AutoCreationError> {
        let mut conn: Connection = self.create_mutable_connection().await?;
        let pong: String =
            redis::cmd("PING")
                .query(&mut conn)
                .map_err(|error: redis::RedisError| {
                    AutoCreationError::ConnectionFailed(format!("Redis PING failed: {error}"))
                })?;
        if pong != "PONG" {
            return Err(AutoCreationError::ConnectionFailed(
                "Redis PING returned unexpected response".to_string(),
            ));
        }
        let info: String = redis::cmd("INFO").arg("server").query(&mut conn).map_err(
            |error: redis::RedisError| {
                AutoCreationError::DatabaseError(format!(
                    "Failed to get Redis server info: {error}"
                ))
            },
        )?;
        if info.contains("redis_version:") {
            AutoCreationLogger::log_connection_verification(
                crate::database::PluginType::Redis,
                "default",
                true,
                None,
            )
            .await;
        }
        Ok(())
    }

    async fn setup_redis_namespace(&self) -> Result<Vec<String>, AutoCreationError> {
        let mut setup_operations: Vec<String> = Vec::new();
        let mut conn: Connection = self.create_mutable_connection().await?;
        let app_key: &str = "hyperlane:initialized";
        let init_value: &str = "true";
        let exists: bool = redis::cmd("EXISTS")
            .arg(&app_key)
            .query(&mut conn)
            .map_err(|error: redis::RedisError| {
                AutoCreationError::DatabaseError(format!(
                    "Failed to check Redis key existence: {error}"
                ))
            })?;
        if !exists {
            let _: () = redis::cmd("SET")
                .arg(&app_key)
                .arg(init_value)
                .query(&mut conn)
                .map_err(|error: redis::RedisError| {
                    AutoCreationError::DatabaseError(format!(
                        "Failed to set Redis initialization key: {error}"
                    ))
                })?;
            setup_operations.push("hyperlane:initialized".to_string());
            let config_key: &str = "hyperlane:config:version";
            let _: () = redis::cmd("SET")
                .arg(&config_key)
                .arg("1.0.0")
                .query(&mut conn)
                .map_err(|error: redis::RedisError| {
                    AutoCreationError::DatabaseError(format!(
                        "Failed to set Redis config key: {error}"
                    ))
                })?;
            setup_operations.push("hyperlane:config:version".to_string());
        }
        Ok(setup_operations)
    }
}

impl DatabaseAutoCreation for RedisAutoCreation {
    fn create_database_if_not_exists(
        &self,
    ) -> impl std::future::Future<Output = Result<bool, AutoCreationError>> + Send {
        async move {
            self.validate_redis_server().await?;

            AutoCreationLogger::log_database_exists("default", crate::database::PluginType::Redis)
                .await;

            Ok(false)
        }
    }

    fn create_tables_if_not_exist(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<String>, AutoCreationError>> + Send {
        async move {
            let setup_operations = self.setup_redis_namespace().await?;

            if !setup_operations.is_empty() {
                AutoCreationLogger::log_tables_created(
                    &setup_operations,
                    "default",
                    crate::database::PluginType::Redis,
                )
                .await;
            } else {
                AutoCreationLogger::log_tables_created(
                    &[],
                    "default",
                    crate::database::PluginType::Redis,
                )
                .await;
            }

            Ok(setup_operations)
        }
    }

    fn verify_connection(
        &self,
    ) -> impl std::future::Future<Output = Result<(), AutoCreationError>> + Send {
        async move {
            match self.validate_redis_server().await {
                Ok(_) => {
                    AutoCreationLogger::log_connection_verification(
                        crate::database::PluginType::Redis,
                        "default",
                        true,
                        None,
                    )
                    .await;
                    Ok(())
                }
                Err(error) => {
                    AutoCreationLogger::log_connection_verification(
                        crate::database::PluginType::Redis,
                        "default",
                        false,
                        Some(&error.to_string()),
                    )
                    .await;
                    Err(error)
                }
            }
        }
    }
}
