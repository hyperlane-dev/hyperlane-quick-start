/// Display name constant for MySQL plugin type.
pub const MYSQL_DISPLAY_NAME: &str = "MySQL";

/// Display name constant for PostgreSQL plugin type.
pub const POSTGRESQL_DISPLAY_NAME: &str = "PostgreSQL";

/// Display name constant for Redis plugin type.
pub const REDIS_DISPLAY_NAME: &str = "Redis";

/// Status label for an auto-creation result that applied schema changes.
pub const AUTO_CREATION_STATUS_INITIALIZED_WITH_CHANGES: &str = "initialized with changes";

/// Status label for an auto-creation result that found no changes to apply.
pub const AUTO_CREATION_STATUS_VERIFIED: &str = "verified";

/// Error message reported when auto-creation validation finds no MySQL instance configured.
pub const MISSING_MYSQL_INSTANCE_ERROR: &str = "At least one MySQL instance is required";

/// Error message reported when auto-creation validation finds no PostgreSQL instance configured.
pub const MISSING_POSTGRESQL_INSTANCE_ERROR: &str = "At least one PostgreSQL instance is required";

/// Error message reported when auto-creation validation finds no Redis instance configured.
pub const MISSING_REDIS_INSTANCE_ERROR: &str = "At least one Redis instance is required";

/// Placeholder database name used when no default instance can be resolved.
pub const UNKNOWN_DATABASE_NAME: &str = "unknown";

/// Placeholder database name used for Redis, which has no separately named database.
pub const DEFAULT_REDIS_DATABASE_NAME: &str = "default";

/// Placeholder error message used when no error detail is available.
pub const UNKNOWN_ERROR_MESSAGE: &str = "Unknown error";
