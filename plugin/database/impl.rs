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

impl AutoCreationErrorHandler {
    pub fn standardize_error_message(
        error: &AutoCreationError,
        plugin_type: PluginType,
        operation: &str,
    ) -> String {
        let prefix: String = format!("[{}] {operation}", plugin_type.as_str().to_uppercase());
        match error {
            AutoCreationError::InsufficientPermissions(msg) => {
                format!("{prefix} - PERMISSION DENIED: {msg}")
            }
            AutoCreationError::ConnectionFailed(msg) => {
                format!("{prefix} - CONNECTION FAILED: {msg}")
            }
            AutoCreationError::SchemaError(msg) => {
                format!("{prefix} - SCHEMA ERROR: {msg}")
            }
            AutoCreationError::Timeout(msg) => {
                format!("{prefix} - TIMEOUT: {msg}")
            }
            AutoCreationError::DatabaseError(msg) => {
                format!("{prefix} - DATABASE ERROR: {msg}")
            }
        }
    }

    pub fn get_log_level(error: &AutoCreationError) -> &'static str {
        match error {
            AutoCreationError::InsufficientPermissions(_) => "WARN",
            AutoCreationError::ConnectionFailed(_) => "ERROR",
            AutoCreationError::SchemaError(_) => "WARN",
            AutoCreationError::Timeout(_) => "WARN",
            AutoCreationError::DatabaseError(_) => "ERROR",
        }
    }

    pub fn get_recovery_suggestion(error: &AutoCreationError, plugin_type: PluginType) -> String {
        match error {
            AutoCreationError::InsufficientPermissions(_) => {
                format!(
                    "Grant CREATE DATABASE and CREATE TABLE permissions to the {plugin_type} user, or disable auto-creation"
                )
            }
            AutoCreationError::ConnectionFailed(_) => {
                format!(
                    "Check {plugin_type} server connectivity, credentials, and network configuration"
                )
            }
            AutoCreationError::SchemaError(_) => {
                format!("Review {plugin_type} table schema definitions and database compatibility")
            }
            AutoCreationError::Timeout(_) => {
                format!(
                    "Increase AUTO_CREATION_TIMEOUT_SECONDS or check {plugin_type} server performance"
                )
            }
            AutoCreationError::DatabaseError(_) => {
                format!("Check {plugin_type} server logs and database configuration")
            }
        }
    }

    pub fn create_error_context(
        error: &AutoCreationError,
        plugin_type: PluginType,
        operation: &str,
        database_name: Option<&str>,
    ) -> ErrorContext {
        ErrorContext {
            plugin_name: plugin_type.to_string(),
            operation: operation.to_string(),
            database_name: database_name.map(|name: &str| name.to_string()),
            error_type: format!("{error:?}")
                .split('(')
                .next()
                .unwrap_or("Unknown")
                .to_string(),
            error_message: error.user_message().to_string(),
            should_continue: error.should_continue(),
            log_level: Self::get_log_level(error).to_string(),
            recovery_suggestion: Self::get_recovery_suggestion(error, plugin_type),
            timestamp: std::time::SystemTime::now(),
        }
    }

    pub async fn handle_error(
        error: &AutoCreationError,
        plugin_type: PluginType,
        operation: &str,
        database_name: Option<&str>,
    ) -> bool {
        let context: ErrorContext =
            Self::create_error_context(error, plugin_type, operation, database_name);
        let _standardized_message: String =
            Self::standardize_error_message(error, plugin_type, operation);
        AutoCreationLogger::log_auto_creation_error(error, operation, plugin_type, database_name)
            .await;
        let recovery_message: String = format!(
            "[{}] RECOVERY SUGGESTION: {}",
            plugin_type.as_str().to_uppercase(),
            context.recovery_suggestion
        );
        if error.should_continue() {
            log_info(&recovery_message).await;
        } else {
            log_error(&recovery_message).await;
        }
        error.should_continue()
    }

    pub fn from_database_error(error: &str, plugin_type: PluginType) -> AutoCreationError {
        let error_lower: String = error.to_lowercase();
        if error_lower.contains("access denied")
            || error_lower.contains("permission denied")
            || error_lower.contains("authentication failed")
            || error_lower.contains("must be owner")
            || error_lower.contains("noauth")
        {
            return AutoCreationError::InsufficientPermissions(format!(
                "{plugin_type} permission error: {error}"
            ));
        }
        if error_lower.contains("connection refused")
            || error_lower.contains("connection failed")
            || error_lower.contains("timeout")
            || error_lower.contains("network")
            || error_lower.contains("host")
        {
            return AutoCreationError::ConnectionFailed(format!(
                "{plugin_type} connection error: {error}"
            ));
        }
        if error_lower.contains("syntax error")
            || error_lower.contains("invalid")
            || error_lower.contains("constraint")
            || error_lower.contains("foreign key")
            || error_lower.contains("duplicate")
        {
            return AutoCreationError::SchemaError(format!("{plugin_type} schema error: {error}"));
        }
        AutoCreationError::DatabaseError(format!("{plugin_type} database error: {error}"))
    }
}

impl ErrorContext {
    pub fn format_for_log(&self) -> String {
        format!(
            "[{}] {} failed - Type: {}, Database: {}, Continue: {}, Suggestion: {}",
            self.plugin_name,
            self.operation,
            self.error_type,
            self.database_name.as_deref().unwrap_or("N/A"),
            self.should_continue,
            self.recovery_suggestion
        )
    }
}

impl AutoCreationResult {
    pub fn new() -> Self {
        Self {
            database_created: false,
            tables_created: Vec::new(),
            errors: Vec::new(),
            duration: Duration::from_secs(0),
        }
    }

    pub fn has_changes(&self) -> bool {
        self.database_created || !self.tables_created.is_empty()
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl Default for AutoCreationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl TableSchema {
    pub fn new(name: String, sql: String) -> Self {
        Self {
            name,
            sql,
            dependencies: Vec::new(),
        }
    }

    pub fn with_dependency(mut self, dependency: String) -> Self {
        self.dependencies.push(dependency);
        self
    }
}

impl DatabaseSchema {
    pub fn new() -> Self {
        Self {
            tables: Vec::new(),
            indexes: Vec::new(),
            constraints: Vec::new(),
        }
    }

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

impl Default for DatabaseSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoCreationConfig {
    pub fn get_env() -> &'static crate::env::EnvConfig {
        crate::env::get_global_env_config()
    }

    pub fn validate() -> Result<(), String> {
        let env: &'static EnvConfig = Self::get_env();
        if env.enable_mysql {
            if env.mysql_host.is_empty() {
                return Err("MySQL host is required when auto-creation is enabled".to_string());
            }
            if env.mysql_username.is_empty() {
                return Err("MySQL username is required when auto-creation is enabled".to_string());
            }
            if env.mysql_database.is_empty() {
                return Err(
                    "MySQL database name is required when auto-creation is enabled".to_string(),
                );
            }
        }
        if env.enable_postgresql {
            if env.postgresql_host.is_empty() {
                return Err("PostgreSQL host is required when auto-creation is enabled".to_string());
            }
            if env.postgresql_username.is_empty() {
                return Err(
                    "PostgreSQL username is required when auto-creation is enabled".to_string(),
                );
            }
            if env.postgresql_database.is_empty() {
                return Err(
                    "PostgreSQL database name is required when auto-creation is enabled"
                        .to_string(),
                );
            }
        }
        if env.enable_redis && env.redis_host.is_empty() {
            return Err("Redis host is required when auto-creation is enabled".to_string());
        }
        Ok(())
    }

    pub fn get_summary() -> String {
        let env: &'static EnvConfig = Self::get_env();
        format!(
            "Auto-creation config: MySQL={}, PostgreSQL={}, Redis={}",
            env.enable_mysql, env.enable_postgresql, env.enable_redis
        )
    }

    pub fn for_plugin(plugin_name: &str) -> PluginAutoCreationConfig {
        PluginAutoCreationConfig {
            plugin_name: plugin_name.to_string(),
        }
    }
}

impl PluginAutoCreationConfig {
    pub fn is_plugin_enabled(&self) -> bool {
        let env: &'static EnvConfig = AutoCreationConfig::get_env();
        if let Ok(plugin_type) = PluginType::from_str(&self.plugin_name) {
            match plugin_type {
                PluginType::MySQL => env.enable_mysql,
                PluginType::PostgreSQL => env.enable_postgresql,
                PluginType::Redis => env.enable_redis,
            }
        } else {
            false
        }
    }

    pub fn get_database_name(&self) -> String {
        let env: &'static EnvConfig = AutoCreationConfig::get_env();
        if let Ok(plugin_type) = PluginType::from_str(&self.plugin_name) {
            match plugin_type {
                PluginType::MySQL => env.mysql_database.clone(),
                PluginType::PostgreSQL => env.postgresql_database.clone(),
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
                    env.mysql_host, env.mysql_port, env.mysql_database
                ),
                PluginType::PostgreSQL => format!(
                    "{}:{}:{}",
                    env.postgresql_host, env.postgresql_port, env.postgresql_database
                ),
                PluginType::Redis => format!("{}:{}", env.redis_host, env.redis_port),
            }
        } else {
            "unknown".to_string()
        }
    }
}

impl AutoCreationLogger {
    pub async fn log_database_created(database_name: &str, plugin_type: PluginType) {
        let message: String = format!(
            "[AUTO-CREATION] Successfully created database '{database_name}' for {plugin_type} plugin"
        );
        log_info(&message).await;
    }

    pub async fn log_database_exists(database_name: &str, plugin_type: PluginType) {
        let message: String = format!(
            "[AUTO-CREATION] Database '{database_name}' already exists for {plugin_type} plugin"
        );
        log_debug(&message).await;
    }

    pub async fn log_table_created(table_name: &str, database_name: &str, plugin_type: PluginType) {
        let message: String = format!(
            "[AUTO-CREATION] Successfully created table '{table_name}' in database '{database_name}' for {plugin_type} plugin"
        );
        log_info(&message).await;
    }

    pub async fn log_table_exists(table_name: &str, database_name: &str, plugin_type: PluginType) {
        let message: String = format!(
            "[AUTO-CREATION] Table '{table_name}' already exists in database '{database_name}' for {plugin_type} plugin"
        );
        log_debug(&message).await;
    }

    pub async fn log_tables_created(
        tables: &[String],
        database_name: &str,
        plugin_type: PluginType,
    ) {
        if tables.is_empty() {
            let message: String = format!(
                "[AUTO-CREATION] No new tables needed for database '{database_name}' in {plugin_type} plugin"
            );
            log_debug(&message).await;
        } else {
            let message: String = format!(
                "[AUTO-CREATION] Created {} tables in database '{database_name}' for {plugin_type} plugin: {}",
                tables.len(),
                tables.join(", ")
            );
            log_info(&message).await;
        }
    }

    pub async fn log_auto_creation_disabled(plugin_type: PluginType, reason: &str) {
        let message: String =
            format!("[AUTO-CREATION] Auto-creation disabled for {plugin_type} plugin: {reason}");
        log_info(&message).await;
    }

    pub async fn log_auto_creation_error(
        error: &AutoCreationError,
        operation: &str,
        plugin_type: PluginType,
        context: Option<&str>,
    ) {
        let context_str: String = context.map(|c| format!(" ({c})")).unwrap_or_default();
        let message: String = format!(
            "[AUTO-CREATION] {operation} failed for {plugin_type} plugin{context_str}: {error}"
        );

        if error.should_continue() {
            log_error(&format!("{message} - Continuing with existing resources")).await;
        } else {
            log_error(&format!("{message} - Fatal error, cannot continue")).await;
        }
    }

    pub async fn log_auto_creation_timeout(
        operation: &str,
        plugin_type: PluginType,
        timeout_seconds: u64,
    ) {
        let message: String = format!(
            "[AUTO-CREATION] {operation} timed out after {timeout_seconds} seconds for {plugin_type} plugin - Continuing with existing resources"
        );
        log_error(&message).await;
    }

    pub async fn log_auto_creation_start(plugin_type: PluginType, database_name: &str) {
        let message = format!(
            "[AUTO-CREATION] Starting auto-creation process for {plugin_type} plugin, database '{database_name}'"
        );
        log_debug(&message).await;
    }

    pub async fn log_auto_creation_complete(plugin_type: PluginType, result: &AutoCreationResult) {
        let message: String = if result.has_changes() {
            format!(
                "[AUTO-CREATION] Completed for {plugin_type} plugin - Database created: {}, Tables created: {} ({}), Duration: {:?}",
                result.database_created,
                result.tables_created.len(),
                result.tables_created.join(", "),
                result.duration
            )
        } else {
            format!(
                "[AUTO-CREATION] Completed for {plugin_type} plugin - No changes needed, Duration: {:?}",
                result.duration
            )
        };

        if result.has_errors() {
            log_error(&format!(
                "{message} - Errors occurred: {}",
                result.errors.join("; ")
            ))
            .await;
        } else {
            log_info(&message).await;
        }
    }

    pub async fn log_connection_verification(
        plugin_type: PluginType,
        database_name: &str,
        success: bool,
        error: Option<&str>,
    ) {
        let message = if success {
            format!(
                "[AUTO-CREATION] Connection verification successful for {plugin_type} plugin, database '{database_name}'"
            )
        } else {
            format!(
                "[AUTO-CREATION] Connection verification failed for {plugin_type} plugin, database '{database_name}': {}",
                error.unwrap_or("Unknown error")
            )
        };

        if success {
            log_debug(&message).await;
        } else {
            log_error(&message).await;
        }
    }

    pub async fn log_permission_issue(
        plugin_type: PluginType,
        operation: &str,
        database_name: &str,
        error_details: &str,
    ) {
        let message = format!(
            "[AUTO-CREATION] Insufficient permissions for {operation} in {plugin_type} plugin, database '{database_name}': {error_details} - Continuing with existing resources"
        );
        log_error(&message).await;
    }
}
