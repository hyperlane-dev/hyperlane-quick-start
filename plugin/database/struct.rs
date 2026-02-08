use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct DatabasePlugin;

#[derive(Clone, Data, Debug)]
pub struct ConnectionCache<T: Clone> {
    #[get(type(copy), pub(crate))]
    pub(super) last_attempt: Instant,
    #[get(pub(crate))]
    pub(super) result: Result<T, String>,
}

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct AutoCreationErrorHandler;

#[derive(Clone, Data, Debug)]
pub struct ErrorContext {
    #[get(pub(crate))]
    pub(super) database_name: Option<String>,
    #[get(pub(crate))]
    pub(super) error_message: String,
    #[get(pub(crate))]
    pub(super) error_type: String,
    #[get(pub(crate))]
    pub(super) log_level: String,
    #[get(pub(crate))]
    pub(super) operation: String,
    #[get(pub(crate))]
    pub(super) plugin_name: String,
    #[get(pub(crate))]
    pub(super) recovery_suggestion: String,
    #[get(type(copy), pub(crate))]
    pub(super) should_continue: bool,
    #[get(pub(crate))]
    pub(super) timestamp: std::time::SystemTime,
}

#[derive(Clone, Data, Debug)]
pub struct AutoCreationResult {
    #[get(type(copy), pub(crate))]
    pub(super) database_created: bool,
    #[get(pub(crate))]
    pub(super) duration: Duration,
    #[get(pub(crate))]
    pub(super) errors: Vec<String>,
    #[get(pub(crate))]
    pub(super) tables_created: Vec<String>,
}

#[derive(Clone, Data, Debug, New)]
pub struct TableSchema {
    #[get(pub(crate))]
    pub(super) dependencies: Vec<String>,
    #[get(pub(crate))]
    pub(super) name: String,
    #[get(pub(crate))]
    pub(super) sql: String,
}

#[derive(Clone, Data, Debug, Default)]
pub struct DatabaseSchema {
    #[get(pub(crate))]
    pub(super) constraints: Vec<String>,
    #[get(pub(crate))]
    pub(super) indexes: Vec<String>,
    #[get(pub(crate))]
    pub(super) tables: Vec<TableSchema>,
}

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct AutoCreationConfig;

#[derive(Clone, Data, Debug, Default)]
pub struct PluginAutoCreationConfig {
    #[get(pub(crate))]
    pub(super) plugin_name: String,
}

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct AutoCreationLogger;
