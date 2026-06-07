use super::*;

/// Implementation of `Display` for `PluginType`, converting the plugin type to its display name.
impl fmt::Display for PluginType {
    /// Formats the `PluginType` as its display name string.
    ///
    /// # Arguments
    ///
    /// - `&self`: The plugin type instance.
    /// - `&mut fmt::Formatter<'_>`: The formatter.
    ///
    /// # Returns
    ///
    /// - `fmt::Result`: The result of the formatting operation.
    #[instrument_trace]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MySQL => write!(f, "{}", MYSQL_DISPLAY_NAME),
            Self::PostgreSQL => write!(f, "{}", POSTGRESQL_DISPLAY_NAME),
            Self::Redis => write!(f, "{}", REDIS_DISPLAY_NAME),
        }
    }
}

/// Implementation of `FromStr` for `PluginType`, parsing a display name string into a plugin type.
impl FromStr for PluginType {
    type Err = ();

    /// Parses a display name string into a `PluginType`.
    ///
    /// # Arguments
    ///
    /// - `&str`: The string to parse.
    ///
    /// # Returns
    ///
    /// - `Result<Self, ()>`: The parsed plugin type, or an error if the string is not recognized.
    #[instrument_trace]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            MYSQL_DISPLAY_NAME => Ok(Self::MySQL),
            POSTGRESQL_DISPLAY_NAME => Ok(Self::PostgreSQL),
            REDIS_DISPLAY_NAME => Ok(Self::Redis),
            _ => Err(()),
        }
    }
}

/// Implementation of `Display` for `AutoCreationError`, providing a human-readable error message.
impl std::fmt::Display for AutoCreationError {
    /// Formats the `AutoCreationError` as a human-readable error message string.
    ///
    /// # Arguments
    ///
    /// - `&self`: The error instance.
    /// - `&mut std::fmt::Formatter<'_>`: The formatter.
    ///
    /// # Returns
    ///
    /// - `std::fmt::Result`: The result of the formatting operation.
    #[instrument_trace]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientPermissions(msg) => {
                write!(f, "Insufficient permissions {msg}")
            }
            Self::ConnectionFailed(msg) => write!(f, "Connection failed {msg}"),
            Self::SchemaError(msg) => write!(f, "Schema error {msg}"),
            Self::Timeout(msg) => write!(f, "Timeout {msg}"),
            Self::DatabaseError(msg) => write!(f, "Database error {msg}"),
        }
    }
}

/// Implementation of `std::error::Error` for `AutoCreationError`, enabling use with the standard error trait.
impl std::error::Error for AutoCreationError {}

/// Implementation of utility methods for `AutoCreationError`.
impl AutoCreationError {
    /// Determines whether the auto-creation process should continue after this error.
    ///
    /// # Returns
    ///
    /// - `bool`: True if the process should continue, false if it should abort.
    #[instrument_trace]
    pub fn should_continue(&self) -> bool {
        match self {
            Self::InsufficientPermissions(_) => true,
            Self::ConnectionFailed(_) => false,
            Self::SchemaError(_) => true,
            Self::Timeout(_) => true,
            Self::DatabaseError(_) => true,
        }
    }

    /// Returns the user-facing error message from this error.
    ///
    /// # Returns
    ///
    /// - `&str`: The error message string.
    #[instrument_trace]
    pub fn user_message(&self) -> &str {
        match self {
            Self::InsufficientPermissions(msg) => msg,
            Self::ConnectionFailed(msg) => msg,
            Self::SchemaError(msg) => msg,
            Self::Timeout(msg) => msg,
            Self::DatabaseError(msg) => msg,
        }
    }
}

/// Implementation of builder methods for `TableSchema`.
impl TableSchema {
    /// Adds a dependency table name to this table schema.
    ///
    /// # Arguments
    ///
    /// - `String`: The name of the dependency table.
    ///
    /// # Returns
    ///
    /// - `Self`: The updated table schema with the added dependency.
    #[instrument_trace]
    pub fn with_dependency(mut self, dependency: String) -> Self {
        self.get_mut_dependencies().push(dependency);
        self
    }
}

/// Implementation of database connection timeout and auto-creation methods for `DatabasePlugin`.
impl DatabasePlugin {
    /// Returns the connection timeout duration from the environment variable.
    ///
    /// # Returns
    ///
    /// - `Duration`: The connection timeout duration.
    ///
    /// # Panics
    ///
    /// Panics if the `DB_CONNECTION_TIMEOUT_MILLIS` environment variable is not set or invalid.
    #[instrument_trace]
    pub fn get_connection_timeout_duration() -> Duration {
        let timeout_millis: u64 = var(ENV_KEY_DB_CONNECTION_TIMEOUT_MILLIS)
            .ok()
            .and_then(|value: String| value.parse::<u64>().ok())
            .unwrap_or_else(|| {
                panic!(
                    "Environment variable {} is not set or invalid",
                    ENV_KEY_DB_CONNECTION_TIMEOUT_MILLIS
                )
            });
        Duration::from_millis(timeout_millis)
    }

    /// Returns the retry interval duration from the environment variable.
    ///
    /// # Returns
    ///
    /// - `Duration`: The retry interval duration.
    ///
    /// # Panics
    ///
    /// Panics if the `DB_RETRY_INTERVAL_MILLIS` environment variable is not set or invalid.
    #[instrument_trace]
    pub fn get_retry_duration() -> Duration {
        let millis: u64 = var(ENV_KEY_DB_RETRY_INTERVAL_MILLIS)
            .ok()
            .and_then(|value: String| value.parse::<u64>().ok())
            .unwrap_or_else(|| {
                panic!(
                    "Environment variable {} is not set or invalid",
                    ENV_KEY_DB_RETRY_INTERVAL_MILLIS
                )
            });
        Duration::from_millis(millis)
    }

    /// Initializes auto-creation for all configured database plugins without a custom schema.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error message on failure.
    #[instrument_trace]
    pub async fn initialize_auto_creation() -> Result<(), String> {
        Self::initialize_auto_creation_with_schema(None, None, None).await
    }

    /// Initializes auto-creation for all configured database plugins with optional schemas.
    ///
    /// # Arguments
    ///
    /// - `Option<DatabaseSchema>`: The optional MySQL schema for table creation.
    /// - `Option<DatabaseSchema>`: The optional PostgreSQL schema for table creation.
    /// - `Option<()>`: The optional Redis schema (currently unused).
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error message on failure.
    #[instrument_trace]
    pub async fn initialize_auto_creation_with_schema(
        mysql_schema: Option<DatabaseSchema>,
        postgresql_schema: Option<DatabaseSchema>,
        _redis_schema: Option<()>,
    ) -> Result<(), String> {
        if let Err(error) = AutoCreationConfig::validate() {
            return Err(format!(
                "Auto-creation configuration validation failed {error}"
            ));
        }
        let env: &'static EnvConfig = EnvPlugin::get_or_init();
        let mut initialization_results: Vec<String> = vec![];
        for instance in env.get_mysql_instances() {
            match MySqlPlugin::perform_auto_creation(instance, mysql_schema.clone()).await {
                Ok(result) => {
                    initialization_results.push(format!(
                        "MySQL ({})  {}",
                        instance.get_name(),
                        if result.has_changes() {
                            "initialized with changes"
                        } else {
                            "verified"
                        }
                    ));
                }
                Err(error) => {
                    if !error.should_continue() {
                        return Err(format!(
                            "MySQL ({}) auto-creation failed {error}",
                            instance.get_name()
                        ));
                    }
                    initialization_results.push(format!(
                        "MySQL ({}) : failed but continuing ({error})",
                        instance.get_name()
                    ));
                }
            }
        }
        for instance in env.get_postgresql_instances() {
            match PostgreSqlPlugin::perform_auto_creation(instance, postgresql_schema.clone()).await
            {
                Ok(result) => {
                    initialization_results.push(format!(
                        "PostgreSQL ({})  {}",
                        instance.get_name(),
                        if result.has_changes() {
                            "initialized with changes"
                        } else {
                            "verified"
                        }
                    ));
                }
                Err(error) => {
                    if !error.should_continue() {
                        return Err(format!(
                            "PostgreSQL ({}) auto-creation failed {error}",
                            instance.get_name()
                        ));
                    }
                    initialization_results.push(format!(
                        "PostgreSQL ({}) : failed but continuing ({error})",
                        instance.get_name()
                    ));
                }
            }
        }
        for instance in env.get_redis_instances() {
            match RedisPlugin::perform_auto_creation(instance, None).await {
                Ok(result) => {
                    initialization_results.push(format!(
                        "Redis ({})  {}",
                        instance.get_name(),
                        if result.has_changes() {
                            "initialized with changes"
                        } else {
                            "verified"
                        }
                    ));
                }
                Err(error) => {
                    if !error.should_continue() {
                        return Err(format!(
                            "Redis ({}) auto-creation failed {error}",
                            instance.get_name()
                        ));
                    }
                    initialization_results.push(format!(
                        "Redis ({}) : failed but continuing ({error})",
                        instance.get_name()
                    ));
                }
            }
        }
        if initialization_results.is_empty() {
            info!("[AUTO-CREATION] No plugins enabled for auto-creation");
        } else {
            let results_summary: String = initialization_results.join(", ");
            info!("[AUTO-CREATION] Initialization complete {results_summary}");
        }
        Ok(())
    }
}

/// Implementation of `ConnectionCache` methods for managing cached connection results.
impl<T: Clone> ConnectionCache<T> {
    /// Creates a new connection cache entry with the given result and current timestamp.
    ///
    /// # Arguments
    ///
    /// - `Result<T, String>`: The connection result to cache.
    ///
    /// # Returns
    ///
    /// - `Self`: A new connection cache entry.
    #[instrument_trace]
    pub fn new(result: Result<T, String>) -> Self {
        Self {
            result,
            last_attempt: Instant::now(),
        }
    }

    /// Checks whether the cached connection result has expired based on the given duration.
    ///
    /// # Arguments
    ///
    /// - `Duration`: The maximum age of the cached result before it is considered expired.
    ///
    /// # Returns
    ///
    /// - `bool`: True if the cache entry has expired.
    #[instrument_trace]
    pub fn is_expired(&self, duration: Duration) -> bool {
        self.get_last_attempt().elapsed() >= duration
    }

    /// Determines whether a failed connection attempt should be retried based on the retry duration.
    ///
    /// # Arguments
    ///
    /// - `Duration`: The minimum time to wait before retrying.
    ///
    /// # Returns
    ///
    /// - `bool`: True if the connection should be retried (failed and expired).
    #[instrument_trace]
    pub fn should_retry(&self, duration: Duration) -> bool {
        self.try_get_result().is_err() && self.is_expired(duration)
    }
}

/// Implementation of result inspection methods for `AutoCreationResult`.
impl AutoCreationResult {
    /// Checks whether any changes were made during the auto-creation process.
    ///
    /// # Returns
    ///
    /// - `bool`: True if a database was created or tables were created.
    #[instrument_trace]
    pub fn has_changes(&self) -> bool {
        self.get_database_created() || !self.get_tables_created().is_empty()
    }

    /// Checks whether any errors occurred during the auto-creation process.
    ///
    /// # Returns
    ///
    /// - `bool`: True if there are errors in the result.
    #[instrument_trace]
    pub fn has_errors(&self) -> bool {
        !self.get_errors().is_empty()
    }
}

/// Implementation of builder methods for `DatabaseSchema`.
impl DatabaseSchema {
    /// Adds a table schema to the database schema.
    ///
    /// # Arguments
    ///
    /// - `TableSchema`: The table schema to add.
    ///
    /// # Returns
    ///
    /// - `Self`: The updated database schema with the added table.
    #[instrument_trace]
    pub fn add_table(mut self, table: TableSchema) -> Self {
        self.get_mut_tables().push(table);
        self
    }

    /// Adds an index SQL statement to the database schema.
    ///
    /// # Arguments
    ///
    /// - `String`: The index SQL statement to add.
    ///
    /// # Returns
    ///
    /// - `Self`: The updated database schema with the added index.
    #[instrument_trace]
    pub fn add_index(mut self, index: String) -> Self {
        self.get_mut_indexes().push(index);
        self
    }

    /// Adds a constraint SQL statement to the database schema.
    ///
    /// # Arguments
    ///
    /// - `String`: The constraint SQL statement to add.
    ///
    /// # Returns
    ///
    /// - `Self`: The updated database schema with the added constraint.
    #[instrument_trace]
    pub fn add_constraint(mut self, constraint: String) -> Self {
        self.get_mut_constraints().push(constraint);
        self
    }

    /// Adds an initialization data SQL statement to the database schema.
    ///
    /// # Arguments
    ///
    /// - `String`: The init data SQL statement to add.
    ///
    /// # Returns
    ///
    /// - `Self`: The updated database schema with the added init data.
    #[instrument_trace]
    pub fn add_init_data(mut self, init_data: String) -> Self {
        self.get_mut_init_data().push(init_data);
        self
    }

    /// Returns the tables in topological order based on their dependencies.
    ///
    /// Tables with satisfied dependencies are ordered first. If a circular dependency is detected,
    /// the remaining tables are appended in their current order.
    ///
    /// # Returns
    ///
    /// - `Vec<&TableSchema>`: The ordered list of table schema references.
    #[instrument_trace]
    pub fn ordered_tables(&self) -> Vec<&TableSchema> {
        let mut ordered: Vec<&TableSchema> = vec![];
        let mut remaining: Vec<&TableSchema> = self.get_tables().iter().collect();
        while !remaining.is_empty() {
            let mut added_any: bool = false;
            remaining.retain(|table: &&TableSchema| {
                let dependencies_satisfied: bool =
                    table.get_dependencies().iter().all(|dep: &String| {
                        ordered.iter().any(|ordered_table: &&TableSchema| {
                            ordered_table.get_name().as_str() == dep.as_str()
                        })
                    });
                if dependencies_satisfied {
                    ordered.push(table);
                    added_any = true;
                    false
                } else {
                    true
                }
            });
            if !added_any && !remaining.is_empty() {
                for table in remaining {
                    ordered.push(table);
                }
                break;
            }
        }
        ordered
    }
}

/// Implementation of validation and factory methods for `AutoCreationConfig`.
impl AutoCreationConfig {
    /// Validates that at least one instance of each required database plugin is configured.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok if validation passes, or an error message identifying the missing plugin.
    #[instrument_trace]
    pub fn validate() -> Result<(), String> {
        let env: &'static EnvConfig = EnvPlugin::get_or_init();
        if env.get_mysql_instances().is_empty() {
            return Err("At least one MySQL instance is required".to_string());
        }
        if env.get_postgresql_instances().is_empty() {
            return Err("At least one PostgreSQL instance is required".to_string());
        }
        if env.get_redis_instances().is_empty() {
            return Err("At least one Redis instance is required".to_string());
        }
        Ok(())
    }

    /// Creates a `PluginAutoCreationConfig` for the specified plugin name.
    ///
    /// # Arguments
    ///
    /// - `&str`: The name of the plugin.
    ///
    /// # Returns
    ///
    /// - `PluginAutoCreationConfig`: The configuration for the specified plugin.
    #[instrument_trace]
    pub fn for_plugin(plugin_name: &str) -> PluginAutoCreationConfig {
        PluginAutoCreationConfig {
            plugin_name: plugin_name.to_string(),
        }
    }
}

/// Implementation of inspection methods for `PluginAutoCreationConfig`.
impl PluginAutoCreationConfig {
    /// Checks whether the plugin identified by this configuration is enabled and recognized.
    ///
    /// # Returns
    ///
    /// - `bool`: True if the plugin name maps to a valid `PluginType`.
    #[instrument_trace]
    pub fn is_plugin_enabled(&self) -> bool {
        PluginType::from_str(self.get_plugin_name()).is_ok()
    }

    /// Returns the database name associated with this plugin's default instance.
    ///
    /// # Returns
    ///
    /// - `String`: The database name, or "unknown" if no default instance is found.
    #[instrument_trace]
    pub fn get_database_name(&self) -> String {
        let env: &'static EnvConfig = EnvPlugin::get_or_init();
        if let Ok(plugin_type) = PluginType::from_str(self.get_plugin_name()) {
            match plugin_type {
                PluginType::MySQL => {
                    if let Some(instance) = env.get_default_mysql_instance() {
                        instance.get_database().clone()
                    } else {
                        "unknown".to_string()
                    }
                }
                PluginType::PostgreSQL => {
                    if let Some(instance) = env.get_default_postgresql_instance() {
                        instance.get_database().clone()
                    } else {
                        "unknown".to_string()
                    }
                }
                PluginType::Redis => "default".to_string(),
            }
        } else {
            "unknown".to_string()
        }
    }

    /// Returns the connection information (host:port:database) for this plugin's default instance.
    ///
    /// # Returns
    ///
    /// - `String`: The connection information string, or "unknown" if no default instance is found.
    #[instrument_trace]
    pub fn get_connection_info(&self) -> String {
        let env: &'static EnvConfig = EnvPlugin::get_or_init();
        if let Ok(plugin_type) = PluginType::from_str(self.get_plugin_name()) {
            match plugin_type {
                PluginType::MySQL => {
                    if let Some(instance) = env.get_default_mysql_instance() {
                        format!(
                            "{}:{}:{}",
                            instance.get_host(),
                            instance.get_port(),
                            instance.get_database()
                        )
                    } else {
                        "unknown".to_string()
                    }
                }
                PluginType::PostgreSQL => {
                    if let Some(instance) = env.get_default_postgresql_instance() {
                        format!(
                            "{}:{}:{}",
                            instance.get_host(),
                            instance.get_port(),
                            instance.get_database()
                        )
                    } else {
                        "unknown".to_string()
                    }
                }
                PluginType::Redis => {
                    if let Some(instance) = env.get_default_redis_instance() {
                        format!("{}:{}", instance.get_host(), instance.get_port())
                    } else {
                        "unknown".to_string()
                    }
                }
            }
        } else {
            "unknown".to_string()
        }
    }
}

/// Implementation of logging methods for `AutoCreationLogger`, providing standardized log output for auto-creation events.
impl AutoCreationLogger {
    /// Logs the start of an auto-creation operation for a specific plugin and database.
    ///
    /// # Arguments
    ///
    /// - `PluginType`: The type of the database plugin.
    /// - `&str`: The name of the database being created.
    #[instrument_trace]
    pub async fn log_auto_creation_start(plugin_type: PluginType, database_name: &str) {
        info!(
            "[AUTO-CREATION] Starting auto-creation for {plugin_type} database '{database_name}'"
        );
    }

    /// Logs the completion of an auto-creation operation, including any warnings.
    ///
    /// # Arguments
    ///
    /// - `PluginType`: The type of the database plugin.
    /// - `&AutoCreationResult`: The result of the auto-creation operation.
    #[instrument_trace]
    pub async fn log_auto_creation_complete(plugin_type: PluginType, result: &AutoCreationResult) {
        if result.has_errors() {
            info!(
                "[AUTO-CREATION] Auto-creation completed for {plugin_type} with warnings {}",
                result.get_errors().join(", ")
            );
        } else {
            info!("[AUTO-CREATION] Auto-creation completed successfully for {plugin_type}");
        }
    }

    /// Logs an error that occurred during an auto-creation operation.
    ///
    /// # Arguments
    ///
    /// - `&AutoCreationError`: The error that occurred.
    /// - `&str`: The operation that failed.
    /// - `PluginType`: The type of the database plugin.
    /// - `Option<&str>`: The optional database name.
    #[instrument_trace]
    pub async fn log_auto_creation_error(
        error: &AutoCreationError,
        operation: &str,
        plugin_type: PluginType,
        database_name: Option<&str>,
    ) {
        error!(
            "[AUTO-CREATION] {operation} failed for {plugin_type} database '{}' {error}",
            database_name.unwrap_or("unknown")
        );
    }

    /// Logs the result of a connection verification for a specific plugin and database.
    ///
    /// # Arguments
    ///
    /// - `PluginType`: The type of the database plugin.
    /// - `&str`: The name of the database.
    /// - `bool`: Whether the verification was successful.
    /// - `Option<&str>`: The optional error message if verification failed.
    #[instrument_trace]
    pub async fn log_connection_verification(
        plugin_type: PluginType,
        database_name: &str,
        success: bool,
        error: Option<&str>,
    ) {
        if success {
            info!(
                "[AUTO-CREATION] Connection verification successful for {plugin_type} database '{database_name}'"
            );
        } else {
            error!(
                "[AUTO-CREATION] Connection verification failed for {plugin_type} database '{database_name}' {}",
                error.unwrap_or("Unknown error")
            );
        };
    }

    /// Logs that a new database was successfully created for a specific plugin.
    ///
    /// # Arguments
    ///
    /// - `&str`: The name of the database that was created.
    /// - `PluginType`: The type of the database plugin.
    #[instrument_trace]
    pub async fn log_database_created(database_name: &str, plugin_type: PluginType) {
        info!(
            "[AUTO-CREATION] Successfully created database '{database_name}' for {plugin_type} plugin"
        );
    }

    /// Logs that the specified database already exists for a specific plugin.
    ///
    /// # Arguments
    ///
    /// - `&str`: The name of the database.
    /// - `PluginType`: The type of the database plugin.
    #[instrument_trace]
    pub async fn log_database_exists(database_name: &str, plugin_type: PluginType) {
        info!("[AUTO-CREATION] Database '{database_name}' already exists for {plugin_type} plugin");
    }

    /// Logs that a new table was successfully created in the database for a specific plugin.
    ///
    /// # Arguments
    ///
    /// - `&str`: The name of the table that was created.
    /// - `&str`: The name of the database.
    /// - `PluginType`: The type of the database plugin.
    #[instrument_trace]
    pub async fn log_table_created(table_name: &str, database_name: &str, plugin_type: PluginType) {
        info!(
            "[AUTO-CREATION] Successfully created table '{table_name}' in database '{database_name}' for {plugin_type} plugin"
        );
    }

    /// Logs that the specified table already exists in the database for a specific plugin.
    ///
    /// # Arguments
    ///
    /// - `&str`: The name of the table.
    /// - `&str`: The name of the database.
    /// - `PluginType`: The type of the database plugin.
    #[instrument_trace]
    pub async fn log_table_exists(table_name: &str, database_name: &str, plugin_type: PluginType) {
        info!(
            "[AUTO-CREATION] Table '{table_name}' already exists in database '{database_name}' for {plugin_type} plugin"
        );
    }

    /// Logs a summary of all tables created during the auto-creation process.
    ///
    /// # Arguments
    ///
    /// - `&[String]`: The list of table names that were created.
    /// - `&str`: The name of the database.
    /// - `PluginType`: The type of the database plugin.
    #[instrument_trace]
    pub async fn log_tables_created(
        tables: &[String],
        database_name: &str,
        plugin_type: PluginType,
    ) {
        if tables.is_empty() {
            info!(
                "[AUTO-CREATION] No new tables created in database '{database_name}' for {plugin_type} plugin"
            );
        } else {
            info!(
                "[AUTO-CREATION] Created tables [{}] in database '{database_name}' for {plugin_type} plugin",
                tables.join(", ")
            );
        }
    }
}
