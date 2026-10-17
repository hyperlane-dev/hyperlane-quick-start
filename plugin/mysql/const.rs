/// Default name for the MySQL instance when no explicit name is provided.
pub const DEFAULT_MYSQL_INSTANCE_NAME: &str = "mysql_default";

/// Log label for the auto-creation process of an instance.
pub(crate) const AUTO_CREATION_PROCESS_LABEL: &str = "Auto-creation process";

/// Log label for the database creation step.
pub(crate) const DATABASE_CREATION_LABEL: &str = "Database creation";

/// Log label for the table creation step.
pub(crate) const TABLE_CREATION_LABEL: &str = "Table creation";

/// Log label for the index creation step.
pub(crate) const INDEX_CREATION_LABEL: &str = "Index creation";

/// Log label for the constraint creation step.
pub(crate) const CONSTRAINT_CREATION_LABEL: &str = "Constraint creation";

/// Log label for the init data step.
pub(crate) const INIT_DATA_LABEL: &str = "Init data";

/// Log label for the init data insertion step.
pub(crate) const INIT_DATA_INSERTION_LABEL: &str = "Init data insertion";

/// Log label for the connection verification step.
pub(crate) const CONNECTION_VERIFICATION_LABEL: &str = "Connection verification";

/// Substring marking denied access in a driver error.
pub(crate) const ERROR_MARKER_ACCESS_DENIED: &str = "Access denied";

/// Substring marking a permission problem in a driver error.
pub(crate) const ERROR_MARKER_PERMISSION: &str = "permission";

/// Substring marking a connection timeout in a driver error.
pub(crate) const ERROR_MARKER_TIMEOUT: &str = "timeout";

/// Substring marking a refused connection in a driver error.
pub(crate) const ERROR_MARKER_CONNECTION_REFUSED: &str = "Connection refused";

/// Probe statement executed to confirm the connection is alive.
pub(crate) const CONNECTION_PROBE_QUERY: &str = "SELECT 1";
