/// Default name for the Redis instance when no explicit name is provided.
pub const DEFAULT_REDIS_INSTANCE_NAME: &str = "redis_default";

/// Step label for the overall automatic database creation process.
pub const AUTO_CREATION_PROCESS_LABEL: &str = "Auto-creation process";

/// Step label for the database existence validation step.
pub const DATABASE_VALIDATION_LABEL: &str = "Database validation";

/// Step label for the key namespace setup step.
pub const NAMESPACE_SETUP_LABEL: &str = "Namespace setup";

/// Step label for the connection verification step.
pub const CONNECTION_VERIFICATION_LABEL: &str = "Connection verification";

/// Error message reported when the blocking connection task cannot be joined.
pub const CONNECTION_TASK_FAILED_MESSAGE: &str = "Redis connection task failed";

/// Error message marker used to detect a failed authentication attempt.
pub const ERROR_MARKER_AUTHENTICATION_FAILED: &str = "authentication failed";

/// Error message marker used to detect a missing Redis authentication.
pub const ERROR_MARKER_NOAUTH: &str = "NOAUTH";

/// Error message marker used to detect a connection timeout.
pub const ERROR_MARKER_TIMEOUT: &str = "timeout";

/// Error message marker used to detect a refused server connection.
pub const ERROR_MARKER_CONNECTION_REFUSED: &str = "Connection refused";

/// Protocol command used to probe server liveness.
pub const REDIS_COMMAND_PING: &str = "PING";

/// Expected reply to the PING protocol command.
pub const REDIS_RESPONSE_PONG: &str = "PONG";

/// Error message reported when the PING reply is not the expected PONG value.
pub const PING_UNEXPECTED_RESPONSE_MESSAGE: &str = "Redis PING returned unexpected response";

/// Protocol command used to read server metadata.
pub const REDIS_COMMAND_INFO: &str = "INFO";

/// INFO section requested from the Redis server.
pub const REDIS_INFO_SECTION_SERVER: &str = "server";

/// INFO field marking the reported server version.
pub const REDIS_INFO_VERSION_KEY: &str = "redis_version:";

/// Protocol command used to test key existence.
pub const REDIS_COMMAND_EXISTS: &str = "EXISTS";

/// Value stored in the namespace initialization key.
pub const REDIS_INIT_KEY_VALUE: &str = "true";

/// Version value stored in the namespace configuration key.
pub const REDIS_CONFIG_VERSION_VALUE: &str = "1.0.0";
