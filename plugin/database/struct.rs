use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct DatabasePlugin;

#[derive(Clone, Data, Debug)]
pub struct ConnectionCache<T: Clone> {
    #[get(type(copy))]
    pub(super) last_attempt: Instant,
    pub(super) result: Result<T, String>,
}

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct AutoCreationErrorHandler;

#[derive(Clone, Data, Debug, Default)]
pub struct AutoCreationResult {
    #[get(type(copy))]
    pub(super) database_created: bool,
    pub(super) duration: Duration,
    pub(super) errors: Vec<String>,
    pub(super) tables_created: Vec<String>,
}

#[derive(Clone, Data, Debug, New)]
pub struct TableSchema {
    pub(super) dependencies: Vec<String>,
    pub(super) name: String,
    pub(super) sql: String,
}

#[derive(Clone, Data, Debug, Default)]
pub struct DatabaseSchema {
    pub(super) constraints: Vec<String>,
    pub(super) indexes: Vec<String>,
    pub(super) init_data: Vec<String>,
    pub(super) tables: Vec<TableSchema>,
}

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct AutoCreationConfig;

#[derive(Clone, Data, Debug, Default)]
pub struct PluginAutoCreationConfig {
    pub(super) plugin_name: String,
}

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct AutoCreationLogger;
