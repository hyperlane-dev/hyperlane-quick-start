use super::*;

/// Trait for plugins that provide a lazily-initialized singleton instance.
///
/// Implementors must be `Clone`, `Copy`, `Default`, `Send`, `Sync`, and `'static`.
/// The instance is initialized once and shared globally.
pub trait GetOrInit: Clone + Copy + Default + Send + Sync + 'static {
    /// The type of the singleton instance provided by this plugin.
    type Instance: Send + Sync + 'static;

    /// Returns a static reference to the lazily-initialized singleton instance.
    fn get_or_init() -> &'static Self::Instance;
}

/// Trait for database connection plugins that manage connections and auto-creation logic.
///
/// Each plugin must define its instance configuration, auto-creation handler, connection type, and cache type.
pub trait DatabaseConnectionPlugin: Clone + Copy + Default + Send + Sync + 'static {
    /// The configuration type for a database instance.
    type InstanceConfig: Clone + Send + Sync + 'static;

    /// The auto-creation handler type for this database plugin.
    type AutoCreation: DatabaseAutoCreation<InstanceConfig = Self::InstanceConfig>;

    /// The connection type used to interact with the database.
    type Connection: Clone + Send + Sync + 'static;

    /// The cache type for storing connection results.
    type ConnectionCache: Send + Sync + 'static;

    /// Returns the plugin type (MySQL, PostgreSQL, or Redis).
    fn plugin_type() -> PluginType;

    /// Creates a new database connection for the specified instance and schema.
    ///
    /// # Arguments
    ///
    /// - `I`: The instance name identifier.
    /// - `Option<DatabaseSchema>`: The optional database schema for auto-creation.
    ///
    /// # Returns
    ///
    /// - `Result<Self::Connection, String>`: The connection on success, or an error message on failure.
    fn connection_db<I>(
        instance_name: I,
        schema: Option<DatabaseSchema>,
    ) -> impl Future<Output = Result<Self::Connection, String>> + Send
    where
        I: AsRef<str> + Send;

    /// Retrieves an existing or creates a new database connection for the specified instance and schema.
    ///
    /// # Arguments
    ///
    /// - `I`: The instance name identifier.
    /// - `Option<DatabaseSchema>`: The optional database schema for auto-creation.
    ///
    /// # Returns
    ///
    /// - `Result<Self::Connection, String>`: The connection on success, or an error message on failure.
    fn get_connection<I>(
        instance_name: I,
        schema: Option<DatabaseSchema>,
    ) -> impl Future<Output = Result<Self::Connection, String>> + Send
    where
        I: AsRef<str> + Send;

    /// Performs the auto-creation process for the database and its tables.
    ///
    /// # Arguments
    ///
    /// - `&Self::InstanceConfig`: The instance configuration.
    /// - `Option<DatabaseSchema>`: The optional database schema containing table definitions.
    ///
    /// # Returns
    ///
    /// - `Result<AutoCreationResult, AutoCreationError>`: The auto-creation result on success, or an error on failure.
    fn perform_auto_creation(
        instance: &Self::InstanceConfig,
        schema: Option<DatabaseSchema>,
    ) -> impl Future<Output = Result<AutoCreationResult, AutoCreationError>> + Send;
}

/// Trait for handling automatic database and table creation during initialization.
///
/// Implementors define how to create databases, tables, initialize data, and verify connections.
pub trait DatabaseAutoCreation: Clone + Send + Sync + 'static {
    /// The configuration type for the database instance.
    type InstanceConfig;

    /// Creates a new auto-creation handler from the given instance configuration.
    ///
    /// # Arguments
    ///
    /// - `Self::InstanceConfig`: The instance configuration.
    fn new(instance: Self::InstanceConfig) -> Self;

    /// Creates a new auto-creation handler with an explicit database schema.
    ///
    /// # Arguments
    ///
    /// - `Self::InstanceConfig`: The instance configuration.
    /// - `DatabaseSchema`: The database schema containing table definitions.
    fn with_schema(instance: Self::InstanceConfig, schema: DatabaseSchema) -> Self
    where
        Self: Sized;

    /// Creates the database if it does not already exist.
    ///
    /// # Returns
    ///
    /// - `Result<bool, AutoCreationError>`: True if the database was created, false if it already existed.
    fn create_database_if_not_exists(
        &self,
    ) -> impl Future<Output = Result<bool, AutoCreationError>> + Send;

    /// Creates all tables defined in the schema if they do not already exist.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<String>, AutoCreationError>`: A list of table names that were created.
    fn create_tables_if_not_exist(
        &self,
    ) -> impl Future<Output = Result<Vec<String>, AutoCreationError>> + Send;

    /// Initializes data in the database using the init data SQL statements from the schema.
    ///
    /// # Returns
    ///
    /// - `Result<(), AutoCreationError>`: Ok on success, or an error on failure.
    fn init_data(&self) -> impl Future<Output = Result<(), AutoCreationError>> + Send;

    /// Verifies the database connection is working correctly.
    ///
    /// # Returns
    ///
    /// - `Result<(), AutoCreationError>`: Ok if the connection is valid, or an error on failure.
    fn verify_connection(&self) -> impl Future<Output = Result<(), AutoCreationError>> + Send;
}
