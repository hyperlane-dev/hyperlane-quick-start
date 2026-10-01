/// Default name for the PostgreSQL instance when no explicit name is provided.
pub const DEFAULT_POSTGRESQL_INSTANCE_NAME: &str = "postgres_default";

/// Step label for the overall automatic database creation process.
pub const AUTO_CREATION_PROCESS_LABEL: &str = "Auto-creation process";

/// Step label for the database creation step.
pub const DATABASE_CREATION_LABEL: &str = "Database creation";

/// Step label for the table creation step.
pub const TABLE_CREATION_LABEL: &str = "Table creation";

/// Step label for the index creation step.
pub const INDEX_CREATION_LABEL: &str = "Index creation";

/// Step label for the constraint creation step.
pub const CONSTRAINT_CREATION_LABEL: &str = "Constraint creation";

/// Step label for the initialization data step.
pub const INIT_DATA_LABEL: &str = "Init data";

/// Step label for the initialization data insertion step.
pub const INIT_DATA_INSERTION_LABEL: &str = "Init data insertion";

/// Step label for the connection verification step.
pub const CONNECTION_VERIFICATION_LABEL: &str = "Connection verification";

/// Error message marker used to detect a failed authentication attempt.
pub const ERROR_MARKER_AUTHENTICATION_FAILED: &str = "authentication failed";

/// Error message marker used to detect a permission failure.
pub const ERROR_MARKER_PERMISSION: &str = "permission";

/// Error message marker used to detect an explicit permission denial.
pub const ERROR_MARKER_PERMISSION_DENIED: &str = "permission denied";

/// Error message marker used to detect a missing object ownership.
pub const ERROR_MARKER_MUST_BE_OWNER: &str = "must be owner";

/// Error message marker used to detect a pre-existing database object.
pub const ERROR_MARKER_ALREADY_EXISTS: &str = "already exists";

/// Error message marker used to detect a connection timeout.
pub const ERROR_MARKER_TIMEOUT: &str = "timeout";

/// Error message marker used to detect a refused server connection.
pub const ERROR_MARKER_CONNECTION_REFUSED: &str = "Connection refused";

/// Probe statement executed to confirm the connection is alive.
pub const CONNECTION_PROBE_QUERY: &str = "SELECT 1";
