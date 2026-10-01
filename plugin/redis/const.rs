/// Log label for the auto-creation process of an instance.
pub const AUTO_CREATION_PROCESS_LABEL: &str = "Auto-creation process";

/// Log label for the database validation step.
pub const DATABASE_VALIDATION_LABEL: &str = "Database validation";

/// Log label for the Redis namespace setup step.
pub const NAMESPACE_SETUP_LABEL: &str = "Namespace setup";

/// Log label for the connection verification step.
pub const CONNECTION_VERIFICATION_LABEL: &str = "Connection verification";

/// Error message for a failed Redis connection task.
pub const REDIS_CONNECTION_TASK_FAILED_ERROR: &str = "Redis connection task failed";

/// Error message for an unexpected Redis PING reply.
pub const REDIS_PING_UNEXPECTED_RESPONSE_ERROR: &str = "Redis PING returned unexpected response";

/// Substring marking an authentication failure in a driver error.
pub const ERROR_MARKER_AUTHENTICATION_FAILED: &str = "authentication failed";

/// Substring marking a missing Redis password in a driver error.
pub const ERROR_MARKER_NOAUTH: &str = "NOAUTH";

/// Substring marking a refused connection in a driver error.
pub const ERROR_MARKER_CONNECTION_REFUSED: &str = "Connection refused";

/// Substring marking a connection timeout in a driver error.
pub const ERROR_MARKER_TIMEOUT: &str = "timeout";

/// Redis command name for the connectivity probe.
pub const REDIS_COMMAND_PING: &str = "PING";

/// Redis reply expected from a successful PING probe.
pub const REDIS_RESPONSE_PONG: &str = "PONG";

/// Redis command name for the server information query.
pub const REDIS_COMMAND_INFO: &str = "INFO";

/// Redis INFO section holding the server metadata.
pub const REDIS_INFO_SECTION_SERVER: &str = "server";

/// Prefix of the Redis INFO field carrying the version.
pub const REDIS_INFO_FIELD_VERSION_PREFIX: &str = "redis_version:";

/// Redis command name for the key existence check.
pub const REDIS_COMMAND_EXISTS: &str = "EXISTS";

/// Value stored for the Redis namespace initialization key.
pub const REDIS_NAMESPACE_INITIALIZED_VALUE: &str = "true";

/// Version value stored for the Redis config key.
pub const REDIS_CONFIG_VERSION_VALUE: &str = "1.0.0";

/// Default name for the Redis instance when no explicit name is provided.
pub const DEFAULT_REDIS_INSTANCE_NAME: &str = "redis_default";
