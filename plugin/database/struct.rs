use super::*;

/// Database plugin entry point for managing auto-creation and connection operations.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct DatabasePlugin;

/// Cache entry storing a connection result along with the timestamp of the last attempt.
#[derive(Clone, Data, Debug)]
pub struct ConnectionCache<T: Clone> {
    /// The timestamp of the last connection attempt.
    #[get(type(copy))]
    pub(super) last_attempt: Instant,
    /// The result of the connection attempt, either a connection or an error message.
    pub(super) result: Result<T, String>,
}

/// Error handler for auto-creation failures, providing standardized error processing.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct AutoCreationErrorHandler;

/// Result of an auto-creation operation, summarizing the changes and errors encountered.
#[derive(Clone, Data, Debug, Default)]
pub struct AutoCreationResult {
    /// Whether a new database was created during the operation.
    #[get(type(copy))]
    pub(super) database_created: bool,
    /// The duration of the auto-creation operation.
    pub(super) duration: Duration,
    /// A list of error messages encountered during the operation.
    pub(super) errors: Vec<String>,
    /// A list of table names that were created during the operation.
    pub(super) tables_created: Vec<String>,
}

/// Schema definition for a single database table, including its name, SQL, and dependencies.
#[derive(Clone, Data, Debug, New)]
pub struct TableSchema {
    /// A list of table names that this table depends on, used for creation ordering.
    pub(super) dependencies: Vec<String>,
    /// The name of the table.
    pub(super) name: String,
    /// The SQL statement to create the table.
    pub(super) sql: String,
}

/// Schema definition for an entire database, including tables, indexes, constraints, and initialization data.
#[derive(Clone, Data, Debug, Default)]
pub struct DatabaseSchema {
    /// A list of constraint SQL statements to apply after table creation.
    pub(super) constraints: Vec<String>,
    /// A list of index SQL statements to apply after table creation.
    pub(super) indexes: Vec<String>,
    /// A list of SQL statements for initializing data after table creation.
    pub(super) init_data: Vec<String>,
    /// A list of table schemas defining the database structure.
    pub(super) tables: Vec<TableSchema>,
}

/// Configuration for auto-creation validation, ensuring required plugins are enabled.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct AutoCreationConfig;

/// Configuration for a specific plugin's auto-creation behavior.
#[derive(Clone, Data, Debug, Default)]
pub struct PluginAutoCreationConfig {
    /// The name of the plugin this configuration applies to.
    pub(super) plugin_name: String,
}

/// Logger for auto-creation operations, providing standardized logging for creation events.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct AutoCreationLogger;
