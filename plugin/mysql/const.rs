/// Default name for the MySQL instance when no explicit name is provided.
pub const DEFAULT_MYSQL_INSTANCE_NAME: &str = "mysql_default";

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

/// Error message marker used to detect a rejected credential or access failure.
pub const ERROR_MARKER_ACCESS_DENIED: &str = "Access denied";

/// Error message marker used to detect a permission failure.
pub const ERROR_MARKER_PERMISSION: &str = "permission";

/// Error message marker used to detect a connection timeout.
pub const ERROR_MARKER_TIMEOUT: &str = "timeout";

/// Error message marker used to detect a refused server connection.
pub const ERROR_MARKER_CONNECTION_REFUSED: &str = "Connection refused";

/// Probe statement executed to confirm the connection is alive.
pub const CONNECTION_PROBE_QUERY: &str = "SELECT 1";
