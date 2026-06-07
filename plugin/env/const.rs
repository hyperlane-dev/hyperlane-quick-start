/// Environment variable key for the database connection timeout in milliseconds.
pub const ENV_KEY_DB_CONNECTION_TIMEOUT_MILLIS: &str = "DB_CONNECTION_TIMEOUT_MILLIS";

/// Environment variable key for the database retry interval in milliseconds.
pub const ENV_KEY_DB_RETRY_INTERVAL_MILLIS: &str = "DB_RETRY_INTERVAL_MILLIS";

/// Environment variable key for the MySQL instance configuration JSON.
pub const ENV_KEY_MYSQL: &str = "MYSQL";

/// Environment variable key for the PostgreSQL instance configuration JSON.
pub const ENV_KEY_POSTGRESQL: &str = "POSTGRESQL";

/// Environment variable key for the Redis instance configuration JSON.
pub const ENV_KEY_REDIS: &str = "REDIS";

/// Environment variable key for the GPT API URL.
pub const ENV_KEY_GPT_API_URL: &str = "GPT_API_URL";

/// Environment variable key for the GPT API key.
pub const ENV_KEY_GPT_API_KEY: &str = "GPT_API_KEY";

/// Environment variable key for the GPT model name.
pub const ENV_KEY_GPT_MODEL: &str = "GPT_MODEL";

/// Environment variable key for enabling GPT thinking mode.
pub const ENV_KEY_GPT_ENABLE_THINKING: &str = "GPT_ENABLE_THINKING";

/// Environment variable key for the server listening port.
pub const ENV_KEY_SERVER_PORT: &str = "SERVER_PORT";

/// Environment variable key for the server host address.
pub const ENV_KEY_SERVER_HOST: &str = "SERVER_HOST";

/// Environment variable key for the server buffer size.
pub const ENV_KEY_SERVER_BUFFER: &str = "SERVER_BUFFER";

/// Environment variable key for the server log size limit.
pub const ENV_KEY_SERVER_LOG_SIZE: &str = "SERVER_LOG_SIZE";

/// Environment variable key for the server log directory path.
pub const ENV_KEY_SERVER_LOG_DIR: &str = "SERVER_LOG_DIR";

/// Environment variable key for enabling server inner print output.
pub const ENV_KEY_SERVER_INNER_PRINT: &str = "SERVER_INNER_PRINT";

/// Environment variable key for enabling server inner logging.
pub const ENV_KEY_SERVER_INNER_LOG: &str = "SERVER_INNER_LOG";

/// Environment variable key for enabling TCP no-delay on the server socket.
pub const ENV_KEY_SERVER_NODELAY: &str = "SERVER_NODELAY";

/// Environment variable key for the server time-to-idle connection timeout.
pub const ENV_KEY_SERVER_TTI: &str = "SERVER_TTI";

/// Environment variable key for the server PID file path.
pub const ENV_KEY_SERVER_PID_FILE_PATH: &str = "SERVER_PID_FILE_PATH";

/// Environment variable key for the server HTTP request read timeout in milliseconds.
pub const ENV_KEY_SERVER_REQUEST_HTTP_READ_TIMEOUT_MS: &str = "SERVER_REQUEST_HTTP_READ_TIMEOUT_MS";

/// Environment variable key for the server maximum request body size.
pub const ENV_KEY_SERVER_REQUEST_MAX_BODY_SIZE: &str = "SERVER_REQUEST_MAX_BODY_SIZE";

/// YAML key for the services section in Docker Compose configuration.
pub const DOCKER_YAML_SERVICES: &str = "services";

/// YAML key for the environment section in Docker Compose service configuration.
pub const DOCKER_YAML_ENVIRONMENT: &str = "environment";

/// YAML key for the ports section in Docker Compose service configuration.
pub const DOCKER_YAML_PORTS: &str = "ports";

/// YAML key for the command section in Docker Compose service configuration.
pub const DOCKER_YAML_COMMAND: &str = "command";

/// Docker Compose service name for MySQL.
pub const DOCKER_SERVICE_MYSQL: &str = "mysql";

/// Docker Compose service name for PostgreSQL.
pub const DOCKER_SERVICE_POSTGRESQL: &str = "postgresql";

/// Docker Compose service name for Redis.
pub const DOCKER_SERVICE_REDIS: &str = "redis";

/// Docker environment variable key for the MySQL database name.
pub const DOCKER_MYSQL_DATABASE: &str = "MYSQL_DATABASE";

/// Docker environment variable key for the MySQL user name.
pub const DOCKER_MYSQL_USER: &str = "MYSQL_USER";

/// Docker environment variable key for the MySQL password.
pub const DOCKER_MYSQL_PASSWORD: &str = "MYSQL_PASSWORD";

/// Docker environment variable key for the PostgreSQL database name.
pub const DOCKER_POSTGRES_DB: &str = "POSTGRES_DB";

/// Docker environment variable key for the PostgreSQL user name.
pub const DOCKER_POSTGRES_USER: &str = "POSTGRES_USER";

/// Docker environment variable key for the PostgreSQL password.
pub const DOCKER_POSTGRES_PASSWORD: &str = "POSTGRES_PASSWORD";

/// Docker command flag for setting the Redis requirepass password.
pub const DOCKER_REDIS_PASSWORD_FLAG: &str = "--requirepass";
