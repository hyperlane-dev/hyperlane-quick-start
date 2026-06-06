use super::*;

/// Plugin for lazily initializing and accessing the global `EnvConfig` singleton.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct EnvPlugin;

/// Configuration parsed from Docker Compose file, containing connection details for MySQL, PostgreSQL, and Redis services.
#[derive(Clone, Data, Debug, Default)]
pub struct DockerComposeConfig {
    /// The MySQL database name from Docker Compose environment.
    pub(super) mysql_database: Option<String>,
    /// The MySQL password from Docker Compose environment.
    pub(super) mysql_password: Option<String>,
    /// The MySQL host port mapping from Docker Compose ports.
    #[get(type(copy))]
    pub(super) mysql_port: Option<usize>,
    /// The MySQL username from Docker Compose environment.
    pub(super) mysql_username: Option<String>,
    /// The PostgreSQL database name from Docker Compose environment.
    pub(super) postgresql_database: Option<String>,
    /// The PostgreSQL password from Docker Compose environment.
    pub(super) postgresql_password: Option<String>,
    /// The PostgreSQL host port mapping from Docker Compose ports.
    #[get(type(copy))]
    pub(super) postgresql_port: Option<usize>,
    /// The PostgreSQL username from Docker Compose environment.
    pub(super) postgresql_username: Option<String>,
    /// The Redis password from Docker Compose command.
    pub(super) redis_password: Option<String>,
    /// The Redis host port mapping from Docker Compose ports.
    #[get(type(copy))]
    pub(super) redis_port: Option<usize>,
    /// The Redis username from Docker Compose command.
    pub(super) redis_username: Option<String>,
}

/// Application-wide environment configuration containing all database instances and server settings.
#[derive(Clone, Data, Debug, Default)]
pub struct EnvConfig {
    /// The database connection timeout in milliseconds.
    #[get(type(copy))]
    pub(super) db_connection_timeout_millis: u64,
    /// The database retry interval in milliseconds.
    #[get(type(copy))]
    pub(super) db_retry_interval_millis: u64,
    /// The GPT API URL for AI-related features.
    #[get(pub)]
    pub(super) gpt_api_url: String,
    /// The GPT API key for AI-related features.
    #[get(pub)]
    pub(super) gpt_api_key: String,
    /// The GPT model name for AI-related features.
    #[get(pub)]
    pub(super) gpt_model: String,
    /// Whether to enable GPT thinking mode.
    #[get(type(copy))]
    pub(super) gpt_enable_thinking: bool,
    /// The list of MySQL instance configurations.
    pub(super) mysql_instances: Vec<MySqlInstanceConfig>,
    /// The list of PostgreSQL instance configurations.
    pub(super) postgresql_instances: Vec<PostgreSqlInstanceConfig>,
    /// The list of Redis instance configurations.
    pub(super) redis_instances: Vec<RedisInstanceConfig>,
    /// The server listening port.
    #[get(type(copy))]
    pub(super) server_port: u16,
    /// The server host address.
    #[get(pub)]
    pub(super) server_host: String,
    /// The server buffer size.
    #[get(type(copy))]
    pub(super) server_buffer: usize,
    /// The server log size limit.
    #[get(type(copy))]
    pub(super) server_log_size: usize,
    /// The server log directory path.
    #[get(pub)]
    pub(super) server_log_dir: String,
    /// Whether to enable server inner print output.
    #[get(type(copy))]
    pub(super) server_inner_print: bool,
    /// Whether to enable server inner logging.
    #[get(type(copy))]
    pub(super) server_inner_log: bool,
    /// Whether to enable TCP no-delay on the server socket.
    #[get(type(copy))]
    pub(super) server_nodelay: Option<bool>,
    /// The server time-to-idle connection timeout in seconds.
    #[get(type(copy))]
    pub(super) server_tti: Option<u32>,
    /// The server PID file path for process management.
    #[get(pub)]
    pub(super) server_pid_file_path: String,
    /// The server HTTP request read timeout in milliseconds.
    #[get(type(copy))]
    pub(super) server_request_http_read_timeout_ms: u64,
    /// The server maximum request body size in bytes.
    #[get(type(copy))]
    pub(super) server_request_max_body_size: usize,
}

/// Configuration for a single MySQL database instance, parsed from environment variables.
#[derive(Clone, Debug, Default, serde::Deserialize, Data)]
pub struct MySqlInstanceConfig {
    /// The name identifier for this MySQL instance.
    #[serde(rename = "name")]
    pub(super) name: String,
    /// The host address of this MySQL instance.
    #[serde(rename = "host")]
    pub(super) host: String,
    /// The port number of this MySQL instance.
    #[get(type(copy))]
    #[serde(default, rename = "port")]
    pub(super) port: usize,
    /// The database name of this MySQL instance.
    #[serde(rename = "database")]
    pub(super) database: String,
    /// The username for authenticating with this MySQL instance.
    #[serde(rename = "username")]
    pub(super) username: String,
    /// The password for authenticating with this MySQL instance.
    #[serde(rename = "password")]
    pub(super) password: String,
}

/// Configuration for a single PostgreSQL database instance, parsed from environment variables.
#[derive(Clone, Debug, Default, serde::Deserialize, Data)]
pub struct PostgreSqlInstanceConfig {
    /// The name identifier for this PostgreSQL instance.
    #[serde(rename = "name")]
    pub(super) name: String,
    /// The host address of this PostgreSQL instance.
    #[serde(rename = "host")]
    pub(super) host: String,
    /// The port number of this PostgreSQL instance.
    #[get(type(copy))]
    #[serde(default, rename = "port")]
    pub(super) port: usize,
    /// The database name of this PostgreSQL instance.
    #[serde(rename = "database")]
    pub(super) database: String,
    /// The username for authenticating with this PostgreSQL instance.
    #[serde(rename = "username")]
    pub(super) username: String,
    /// The password for authenticating with this PostgreSQL instance.
    #[serde(rename = "password")]
    pub(super) password: String,
}

/// Configuration for a single Redis instance, parsed from environment variables.
#[derive(Clone, Debug, Default, serde::Deserialize, Data)]
pub struct RedisInstanceConfig {
    /// The name identifier for this Redis instance.
    #[serde(rename = "name")]
    pub(super) name: String,
    /// The host address of this Redis instance.
    #[serde(rename = "host")]
    pub(super) host: String,
    /// The port number of this Redis instance.
    #[get(type(copy))]
    #[serde(default, rename = "port")]
    pub(super) port: usize,
    /// The username for authenticating with this Redis instance.
    #[serde(default, rename = "username")]
    pub(super) username: String,
    /// The password for authenticating with this Redis instance.
    #[serde(rename = "password")]
    pub(super) password: String,
}
