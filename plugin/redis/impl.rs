use super::*;

impl RedisPlugin {
    #[instrument_trace]
    fn get_or_init() -> &'static RwLock<RedisConnectionMap> {
        REDIS_CONNECTIONS.get_or_init(|| RwLock::new(HashMap::new()))
    }

    #[instrument_trace]
    pub async fn connection_db<I>(instance_name: I) -> Result<ArcRwLock<Connection>, String>
    where
        I: AsRef<str>,
    {
        let instance_name_str: &str = instance_name.as_ref();
        let env: &'static EnvConfig = EnvPlugin::get_or_init();
        let instance: &RedisInstanceConfig = env
            .get_redis_instance(instance_name_str)
            .ok_or_else(|| format!("Redis instance '{instance_name_str}' not found"))?;
        match Self::perform_auto_creation(instance).await {
            Ok(result) => {
                if result.has_changes() {
                    AutoCreationLogger::log_auto_creation_complete(
                        database::PluginType::Redis,
                        &result,
                    )
                    .await;
                }
            }
            Err(error) => {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Auto-creation process",
                    database::PluginType::Redis,
                    Some(instance.get_name().as_str()),
                )
                .await;
                if !error.should_continue() {
                    return Err(error.to_string());
                }
            }
        }
        let db_url: String = instance.get_connection_url();
        let client: Client = Client::open(db_url).map_err(|error: redis::RedisError| {
            let error_msg: String = error.to_string();
            let instance_name_clone: String = instance_name_str.to_string();
            let error_msg_clone: String = error_msg.clone();
            spawn(async move {
                AutoCreationLogger::log_connection_verification(
                    database::PluginType::Redis,
                    &instance_name_clone,
                    false,
                    Some(&error_msg_clone),
                )
                .await;
            });
            error_msg
        })?;
        let timeout_duration: Duration = DatabasePlugin::get_connection_timeout_duration();
        let timeout_seconds: u64 = timeout_duration.as_secs();
        let connection_task: JoinHandle<Result<Connection, RedisError>> =
            spawn_blocking(move || client.get_connection());
        let connection: Connection = match timeout(timeout_duration, connection_task).await {
            Ok(join_result) => match join_result {
                Ok(result) => result.map_err(|error: redis::RedisError| {
                    let error_msg: String = error.to_string();
                    let instance_name_clone: String = instance_name_str.to_string();
                    let error_msg_clone: String = error_msg.clone();
                    spawn(async move {
                        AutoCreationLogger::log_connection_verification(
                            database::PluginType::Redis,
                            &instance_name_clone,
                            false,
                            Some(&error_msg_clone),
                        )
                        .await;
                    });
                    error_msg
                })?,
                Err(_) => {
                    let error_msg: String = "Redis connection task failed".to_string();
                    let instance_name_clone: String = instance_name_str.to_string();
                    let error_msg_clone: String = error_msg.clone();
                    spawn(async move {
                        AutoCreationLogger::log_connection_verification(
                            database::PluginType::Redis,
                            &instance_name_clone,
                            false,
                            Some(&error_msg_clone),
                        )
                        .await;
                    });
                    return Err(error_msg);
                }
            },
            Err(_) => {
                let error_msg: String =
                    format!("Redis connection timeout after {timeout_seconds} seconds");
                let instance_name_clone: String = instance_name_str.to_string();
                let error_msg_clone: String = error_msg.clone();
                spawn(async move {
                    AutoCreationLogger::log_connection_verification(
                        database::PluginType::Redis,
                        &instance_name_clone,
                        false,
                        Some(&error_msg_clone),
                    )
                    .await;
                });
                return Err(error_msg);
            }
        };
        Ok(arc_rwlock(connection))
    }

    #[instrument_trace]
    pub async fn get_connection<I>(instance_name: I) -> Result<ArcRwLock<Connection>, String>
    where
        I: AsRef<str>,
    {
        let instance_name_str: &str = instance_name.as_ref();
        let duration: Duration = DatabasePlugin::get_retry_duration();
        {
            if let Some(cache) = Self::get_or_init().read().await.get(instance_name_str) {
                match cache.try_get_result() {
                    Ok(conn) => return Ok(conn.clone()),
                    Err(error) => {
                        if !cache.is_expired(duration) {
                            return Err(error.clone());
                        }
                    }
                }
            }
        }
        let mut connections: RwLockWriteGuard<'_, RedisConnectionMap> =
            Self::get_or_init().write().await;
        if let Some(cache) = connections.get(instance_name_str) {
            match cache.try_get_result() {
                Ok(conn) => return Ok(conn.clone()),
                Err(error) => {
                    if !cache.is_expired(duration) {
                        return Err(error.clone());
                    }
                }
            }
        }
        connections.remove(instance_name_str);
        drop(connections);
        let new_connection: Result<ArcRwLock<Connection>, String> =
            Self::connection_db(instance_name_str).await;
        let mut connections: RwLockWriteGuard<'_, RedisConnectionMap> =
            Self::get_or_init().write().await;
        connections.insert(
            instance_name_str.to_string(),
            ConnectionCache::new(new_connection.clone()),
        );
        new_connection
    }

    #[instrument_trace]
    pub async fn perform_auto_creation(
        instance: &RedisInstanceConfig,
    ) -> Result<AutoCreationResult, AutoCreationError> {
        let start_time: Instant = Instant::now();
        let mut result: AutoCreationResult = AutoCreationResult::default();
        AutoCreationLogger::log_auto_creation_start(
            database::PluginType::Redis,
            instance.get_name(),
        )
        .await;
        let auto_creator: RedisAutoCreation = RedisAutoCreation::new(instance.clone());
        match auto_creator.create_database_if_not_exists().await {
            Ok(created) => {
                result.set_database_created(created);
            }
            Err(error) => {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Database validation",
                    database::PluginType::Redis,
                    Some(instance.get_name().as_str()),
                )
                .await;
                if !error.should_continue() {
                    result.set_duration(start_time.elapsed());
                    return Err(error);
                }
                result.get_mut_errors().push(error.to_string());
            }
        }
        match auto_creator.create_tables_if_not_exist().await {
            Ok(operations) => {
                result.set_tables_created(operations);
            }
            Err(error) => {
                AutoCreationLogger::log_auto_creation_error(
                    &error,
                    "Namespace setup",
                    database::PluginType::Redis,
                    Some(instance.get_name().as_str()),
                )
                .await;
                result.get_mut_errors().push(error.to_string());
            }
        }
        if let Err(error) = auto_creator.verify_connection().await {
            AutoCreationLogger::log_auto_creation_error(
                &error,
                "Connection verification",
                database::PluginType::Redis,
                Some(instance.get_name().as_str()),
            )
            .await;
            if !error.should_continue() {
                result.set_duration(start_time.elapsed());
                return Err(error);
            }
            result.get_mut_errors().push(error.to_string());
        }
        result.set_duration(start_time.elapsed());
        AutoCreationLogger::log_auto_creation_complete(database::PluginType::Redis, &result).await;
        Ok(result)
    }
}

impl Default for RedisAutoCreation {
    #[instrument_trace]
    fn default() -> Self {
        if let Some(instance) = EnvPlugin::get_or_init().get_default_redis_instance() {
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
        let connection_task: JoinHandle<Result<Connection, RedisError>> =
            spawn_blocking(move || client.get_connection());
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
