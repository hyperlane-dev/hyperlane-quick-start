/// Display name constant for MySQL plugin type.
pub const MYSQL_DISPLAY_NAME: &str = "MySQL";

/// Display name constant for PostgreSQL plugin type.
pub const POSTGRESQL_DISPLAY_NAME: &str = "PostgreSQL";

/// Display name constant for Redis plugin type.
pub const REDIS_DISPLAY_NAME: &str = "Redis";

/// Summary text for an initialization that applied schema changes.
pub const INITIALIZATION_RESULT_HAS_CHANGES: &str = "initialized with changes";

/// Summary text for an initialization that found no schema changes.
pub const INITIALIZATION_RESULT_VERIFIED: &str = "verified";

/// Error message raised when no MySQL instance is configured.
pub const MYSQL_INSTANCE_REQUIRED_ERROR: &str = "At least one MySQL instance is required";

/// Error message raised when no PostgreSQL instance is configured.
pub const POSTGRESQL_INSTANCE_REQUIRED_ERROR: &str = "At least one PostgreSQL instance is required";

/// Error message raised when no Redis instance is configured.
pub const REDIS_INSTANCE_REQUIRED_ERROR: &str = "At least one Redis instance is required";

/// Fallback text used when no database name or connection info is known.
pub const UNKNOWN_VALUE: &str = "unknown";

/// Database name of a Redis instance, which has no named databases.
pub const REDIS_DEFAULT_DATABASE_NAME: &str = "default";

/// Placeholder logged when a verification failure has no error text.
pub const LOG_PLACEHOLDER_UNKNOWN_ERROR: &str = "Unknown error";
