/// Log label for the auto-creation process of an instance.
pub const AUTO_CREATION_PROCESS_LABEL: &str = "Auto-creation process";

/// Log label for the database creation step.
pub const DATABASE_CREATION_LABEL: &str = "Database creation";

/// Log label for the table creation step.
pub const TABLE_CREATION_LABEL: &str = "Table creation";

/// Log label for the index creation step.
pub const INDEX_CREATION_LABEL: &str = "Index creation";

/// Log label for the constraint creation step.
pub const CONSTRAINT_CREATION_LABEL: &str = "Constraint creation";

/// Log label for the init data step.
pub const INIT_DATA_LABEL: &str = "Init data";

/// Log label for the init data insertion step.
pub const INIT_DATA_INSERTION_LABEL: &str = "Init data insertion";

/// Log label for the connection verification step.
pub const CONNECTION_VERIFICATION_LABEL: &str = "Connection verification";

/// Substring marking an authentication failure in a driver error.
pub const ERROR_MARKER_AUTHENTICATION_FAILED: &str = "authentication failed";

/// Substring marking a permission problem in a driver error.
pub const ERROR_MARKER_PERMISSION: &str = "permission";

/// Substring marking denied permissions in a PostgreSQL error.
pub const ERROR_MARKER_PERMISSION_DENIED: &str = "permission denied";

/// Substring marking a missing owner role in a PostgreSQL error.
pub const ERROR_MARKER_MUST_BE_OWNER: &str = "must be owner";

/// Substring marking a connection timeout in a driver error.
pub const ERROR_MARKER_TIMEOUT: &str = "timeout";

/// Substring marking a refused connection in a driver error.
pub const ERROR_MARKER_CONNECTION_REFUSED: &str = "Connection refused";

/// Substring marking an already existing object in a PostgreSQL error.
pub const ERROR_MARKER_ALREADY_EXISTS: &str = "already exists";

/// Statement executed to verify a database connection.
pub const CONNECTION_VERIFICATION_SQL: &str = "SELECT 1";

/// Default name for the PostgreSQL instance when no explicit name is provided.
pub const DEFAULT_POSTGRESQL_INSTANCE_NAME: &str = "postgres_default";
