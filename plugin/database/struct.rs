use super::*;

pub struct AutoCreationErrorHandler;

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub plugin_name: String,
    pub operation: String,
    pub database_name: Option<String>,
    pub error_type: String,
    pub error_message: String,
    pub should_continue: bool,
    pub log_level: String,
    pub recovery_suggestion: String,
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug, Clone)]
pub struct AutoCreationResult {
    pub database_created: bool,
    pub tables_created: Vec<String>,
    pub errors: Vec<String>,
    pub duration: Duration,
}

#[derive(Debug, Clone, New)]
pub struct TableSchema {
    pub name: String,
    pub sql: String,
    #[new(skip)]
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DatabaseSchema {
    pub tables: Vec<TableSchema>,
    pub indexes: Vec<String>,
    pub constraints: Vec<String>,
}

pub struct AutoCreationConfig;

pub struct PluginAutoCreationConfig {
    pub plugin_name: String,
}

pub struct AutoCreationLogger;
