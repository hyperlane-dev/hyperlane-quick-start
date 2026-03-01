use super::*;

impl DatabasePlugin {
    #[instrument_trace]
    pub fn get_connection_timeout_duration() -> Duration {
        let timeout_seconds: u64 = std::env::var(ENV_KEY_DB_CONNECTION_TIMEOUT_MILLIS)
            .ok()
            .and_then(|value: String| value.parse::<u64>().ok())
            .unwrap_or(DEFAULT_DB_CONNECTION_TIMEOUT_MILLIS);
        Duration::from_millis(timeout_seconds)
    }

    #[instrument_trace]
    pub fn get_retry_duration() -> Duration {
        let millis: u64 = std::env::var(ENV_KEY_DB_RETRY_INTERVAL_MILLIS)
            .ok()
            .and_then(|value: String| value.parse::<u64>().ok())
            .unwrap_or(DEFAULT_DB_RETRY_INTERVAL_MILLIS);
        Duration::from_millis(millis)
    }

    #[instrument_trace]
    pub async fn initialize_auto_creation() -> Result<(), String> {
        Self::initialize_auto_creation_with_schema(None, None, None).await
    }

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
        let mut initialization_results: Vec<String> = Vec::new();
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

impl<T: Clone> ConnectionCache<T> {
    #[instrument_trace]
    pub fn new(result: Result<T, String>) -> Self {
        Self {
            result,
            last_attempt: Instant::now(),
        }
    }

    #[instrument_trace]
    pub fn is_expired(&self, duration: Duration) -> bool {
        self.get_last_attempt().elapsed() >= duration
    }

    #[instrument_trace]
    pub fn should_retry(&self, duration: Duration) -> bool {
        self.try_get_result().is_err() && self.is_expired(duration)
    }
}

impl fmt::Display for PluginType {
    #[instrument_trace]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MySQL => write!(f, "MySQL"),
            Self::PostgreSQL => write!(f, "PostgreSQL"),
            Self::Redis => write!(f, "Redis"),
        }
    }
}

impl FromStr for PluginType {
    type Err = ();

    #[instrument_trace]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MySQL" => Ok(Self::MySQL),
            "PostgreSQL" => Ok(Self::PostgreSQL),
            "Redis" => Ok(Self::Redis),
            _ => Err(()),
        }
    }
}

impl AutoCreationError {
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

impl std::fmt::Display for AutoCreationError {
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

impl std::error::Error for AutoCreationError {}

impl AutoCreationResult {
    #[instrument_trace]
    pub fn has_changes(&self) -> bool {
        self.get_database_created() || !self.get_tables_created().is_empty()
    }

    #[instrument_trace]
    pub fn has_errors(&self) -> bool {
        !self.get_errors().is_empty()
    }
}

impl Default for AutoCreationResult {
    #[instrument_trace]
    fn default() -> Self {
        Self {
            database_created: false,
            tables_created: Vec::new(),
            errors: Vec::new(),
            duration: Duration::from_secs(0),
        }
    }
}

impl TableSchema {
    #[instrument_trace]
    pub fn with_dependency(mut self, dependency: String) -> Self {
        self.get_mut_dependencies().push(dependency);
        self
    }
}

impl DatabaseSchema {
    #[instrument_trace]
    pub fn add_table(mut self, table: TableSchema) -> Self {
        self.get_mut_tables().push(table);
        self
    }

    #[instrument_trace]
    pub fn add_index(mut self, index: String) -> Self {
        self.get_mut_indexes().push(index);
        self
    }

    #[instrument_trace]
    pub fn add_constraint(mut self, constraint: String) -> Self {
        self.get_mut_constraints().push(constraint);
        self
    }

    #[instrument_trace]
    pub fn add_init_data(mut self, init_data: String) -> Self {
        self.get_mut_init_data().push(init_data);
        self
    }

    #[instrument_trace]
    pub fn ordered_tables(&self) -> Vec<&TableSchema> {
        let mut ordered: Vec<&TableSchema> = Vec::new();
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

impl AutoCreationConfig {
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

    #[instrument_trace]
    pub fn for_plugin(plugin_name: &str) -> PluginAutoCreationConfig {
        PluginAutoCreationConfig {
            plugin_name: plugin_name.to_string(),
        }
    }
}

impl PluginAutoCreationConfig {
    #[instrument_trace]
    pub fn is_plugin_enabled(&self) -> bool {
        PluginType::from_str(self.get_plugin_name()).is_ok()
    }

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

impl AutoCreationLogger {
    #[instrument_trace]
    pub async fn log_auto_creation_start(plugin_type: PluginType, database_name: &str) {
        info!(
            "[AUTO-CREATION] Starting auto-creation for {plugin_type} database '{database_name}'"
        );
    }

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

    #[instrument_trace]
    pub async fn log_database_created(database_name: &str, plugin_type: PluginType) {
        info!(
            "[AUTO-CREATION] Successfully created database '{database_name}' for {plugin_type} plugin"
        );
    }

    #[instrument_trace]
    pub async fn log_database_exists(database_name: &str, plugin_type: PluginType) {
        info!("[AUTO-CREATION] Database '{database_name}' already exists for {plugin_type} plugin");
    }

    #[instrument_trace]
    pub async fn log_table_created(table_name: &str, database_name: &str, plugin_type: PluginType) {
        info!(
            "[AUTO-CREATION] Successfully created table '{table_name}' in database '{database_name}' for {plugin_type} plugin"
        );
    }

    #[instrument_trace]
    pub async fn log_table_exists(table_name: &str, database_name: &str, plugin_type: PluginType) {
        info!(
            "[AUTO-CREATION] Table '{table_name}' already exists in database '{database_name}' for {plugin_type} plugin"
        );
    }

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
