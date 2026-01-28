use super::*;

#[derive(Clone, Debug)]
pub struct ConnectionCache<T: Clone> {
    pub result: Result<T, String>,
    pub last_attempt: Instant,
}

impl<T: Clone> ConnectionCache<T> {
    pub fn new(result: Result<T, String>) -> Self {
        Self {
            result,
            last_attempt: Instant::now(),
        }
    }

    pub fn is_cooldown_expired(&self, cooldown_duration: Duration) -> bool {
        self.last_attempt.elapsed() >= cooldown_duration
    }

    pub fn should_retry(&self, cooldown_duration: Duration) -> bool {
        self.result.is_err() && self.is_cooldown_expired(cooldown_duration)
    }
}

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct AutoCreationErrorHandler;

#[derive(Clone, Data, Debug)]
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

#[derive(Clone, Data, Debug)]
pub struct AutoCreationResult {
    pub database_created: bool,
    pub tables_created: Vec<String>,
    pub errors: Vec<String>,
    pub duration: Duration,
}

#[derive(Clone, Data, Debug, Default, New)]
pub struct TableSchema {
    pub name: String,
    pub sql: String,
    #[new(skip)]
    pub dependencies: Vec<String>,
}

#[derive(Clone, Data, Debug, Default)]
pub struct DatabaseSchema {
    pub tables: Vec<TableSchema>,
    pub indexes: Vec<String>,
    pub constraints: Vec<String>,
}

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct AutoCreationConfig;

#[derive(Clone, Data, Debug, Default)]
pub struct PluginAutoCreationConfig {
    pub plugin_name: String,
}

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct AutoCreationLogger;
