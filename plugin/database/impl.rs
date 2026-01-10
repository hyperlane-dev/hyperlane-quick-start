use super::*;

impl PluginType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MySQL => "MySQL",
            Self::PostgreSQL => "PostgreSQL",
            Self::Redis => "Redis",
        }
    }
}

impl FromStr for PluginType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MySQL" => Ok(Self::MySQL),
            "PostgreSQL" => Ok(Self::PostgreSQL),
            "Redis" => Ok(Self::Redis),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for PluginType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl AutoCreationError {
    pub fn should_continue(&self) -> bool {
        match self {
            Self::InsufficientPermissions(_) => true,
            Self::ConnectionFailed(_) => false,
            Self::SchemaError(_) => true,
            Self::Timeout(_) => true,
            Self::DatabaseError(_) => true,
        }
    }

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientPermissions(msg) => write!(f, "Insufficient permissions: {msg}"),
            Self::ConnectionFailed(msg) => write!(f, "Connection failed: {msg}"),
            Self::SchemaError(msg) => write!(f, "Schema error: {msg}"),
            Self::Timeout(msg) => write!(f, "Timeout: {msg}"),
            Self::DatabaseError(msg) => write!(f, "Database error: {msg}"),
        }
    }
}

impl std::error::Error for AutoCreationError {}

impl AutoCreationResult {
    pub fn has_changes(&self) -> bool {
        self.database_created || !self.tables_created.is_empty()
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl Default for AutoCreationResult {
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
    pub fn with_dependency(mut self, dependency: String) -> Self {
        self.dependencies.push(dependency);
        self
    }
}

impl DatabaseSchema {
    pub fn add_table(mut self, table: TableSchema) -> Self {
        self.tables.push(table);
        self
    }

    pub fn add_index(mut self, index: String) -> Self {
        self.indexes.push(index);
        self
    }

    pub fn add_constraint(mut self, constraint: String) -> Self {
        self.constraints.push(constraint);
        self
    }

    pub fn ordered_tables(&self) -> Vec<&TableSchema> {
        let mut ordered: Vec<&TableSchema> = Vec::new();
        let mut remaining: Vec<&TableSchema> = self.tables.iter().collect();
        while !remaining.is_empty() {
            let mut added_any: bool = false;
            remaining.retain(|table| {
                let dependencies_satisfied = table.dependencies.iter().all(|dep| {
                    ordered
                        .iter()
                        .any(|ordered_table: &&TableSchema| &ordered_table.name == dep)
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
    pub fn get_env() -> &'static env::EnvConfig {
        env::get_global_env_config()
    }

    pub fn validate() -> Result<(), String> {
        let env: &'static EnvConfig = Self::get_env();
        if env.get_mysql_host().is_empty() {
            return Err("MySQL host is required".to_string());
        }
        if env.get_mysql_username().is_empty() {
            return Err("MySQL username is required".to_string());
        }
        if env.get_mysql_database().is_empty() {
            return Err("MySQL database name is required".to_string());
        }
        if env.get_postgresql_host().is_empty() {
            return Err("PostgreSQL host is required".to_string());
        }
        if env.get_postgresql_username().is_empty() {
            return Err("PostgreSQL username is required".to_string());
        }
        if env.get_postgresql_database().is_empty() {
            return Err("PostgreSQL database name is required".to_string());
        }
        if env.get_redis_host().is_empty() {
            return Err("Redis host is required".to_string());
        }
        Ok(())
    }

    pub fn for_plugin(plugin_name: &str) -> PluginAutoCreationConfig {
        PluginAutoCreationConfig {
            plugin_name: plugin_name.to_string(),
        }
    }
}

impl PluginAutoCreationConfig {
    pub fn is_plugin_enabled(&self) -> bool {
        PluginType::from_str(&self.plugin_name).is_ok()
    }

    pub fn get_database_name(&self) -> String {
        let env: &'static EnvConfig = AutoCreationConfig::get_env();
        if let Ok(plugin_type) = PluginType::from_str(&self.plugin_name) {
            match plugin_type {
                PluginType::MySQL => env.get_mysql_database().clone(),
                PluginType::PostgreSQL => env.get_postgresql_database().clone(),
                PluginType::Redis => "default".to_string(),
            }
        } else {
            "unknown".to_string()
        }
    }

    pub fn get_connection_info(&self) -> String {
        let env: &'static EnvConfig = AutoCreationConfig::get_env();
        if let Ok(plugin_type) = PluginType::from_str(&self.plugin_name) {
            match plugin_type {
                PluginType::MySQL => format!(
                    "{}:{}:{}",
                    env.get_mysql_host(),
                    env.get_mysql_port(),
                    env.get_mysql_database()
                ),
                PluginType::PostgreSQL => format!(
                    "{}:{}:{}",
                    env.get_postgresql_host(),
                    env.get_postgresql_port(),
                    env.get_postgresql_database()
                ),
                PluginType::Redis => format!("{}:{}", env.get_redis_host(), env.get_redis_port()),
            }
        } else {
            "unknown".to_string()
        }
    }
}

impl AutoCreationLogger {
    pub async fn log_auto_creation_start(plugin_type: PluginType, database_name: &str) {
        info!(
            "[AUTO-CREATION] Starting auto-creation for {plugin_type} database '{database_name}'"
        );
    }

    pub async fn log_auto_creation_complete(plugin_type: PluginType, result: &AutoCreationResult) {
        if result.has_errors() {
            info!(
                "[AUTO-CREATION] Auto-creation completed for {plugin_type} with warnings: {}",
                result.errors.join(", ")
            );
        } else {
            info!("[AUTO-CREATION] Auto-creation completed successfully for {plugin_type}");
        }
    }

    pub async fn log_auto_creation_error(
        error: &AutoCreationError,
        operation: &str,
        plugin_type: PluginType,
        database_name: Option<&str>,
    ) {
        error!(
            "[AUTO-CREATION] {operation} failed for {plugin_type} database '{}': {error}",
            database_name.unwrap_or("unknown")
        );
    }

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
                "[AUTO-CREATION] Connection verification failed for {plugin_type} database '{database_name}': {}",
                error.unwrap_or("Unknown error")
            );
        };
    }

    pub async fn log_database_created(database_name: &str, plugin_type: PluginType) {
        info!(
            "[AUTO-CREATION] Successfully created database '{database_name}' for {plugin_type} plugin"
        );
    }

    pub async fn log_database_exists(database_name: &str, plugin_type: PluginType) {
        info!("[AUTO-CREATION] Database '{database_name}' already exists for {plugin_type} plugin");
    }

    pub async fn log_table_created(table_name: &str, database_name: &str, plugin_type: PluginType) {
        info!(
            "[AUTO-CREATION] Successfully created table '{table_name}' in database '{database_name}' for {plugin_type} plugin"
        );
    }

    pub async fn log_table_exists(table_name: &str, database_name: &str, plugin_type: PluginType) {
        info!(
            "[AUTO-CREATION] Table '{table_name}' already exists in database '{database_name}' for {plugin_type} plugin"
        );
    }

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
