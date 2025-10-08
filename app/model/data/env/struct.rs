#[derive(Debug, Clone, Default)]
pub struct EnvConfig {
    pub gpt_api_url: String,
    /// GPT model identifier
    pub gtp_model: String,
    /// MySQL database host address
    pub mysql_host: String,
    /// MySQL database port number
    pub mysql_port: u16,
    /// MySQL database name
    pub mysql_database: String,
    /// MySQL database username
    pub mysql_username: String,
    /// MySQL database password
    pub mysql_password: String,
    /// Redis server host address
    pub redis_host: String,
    /// Redis server port number
    pub redis_port: u16,
    /// Redis database username
    pub redis_username: String,
    /// Redis authentication password
    pub redis_password: String,
    /// PostgreSQL database host address
    pub postgresql_host: String,
    /// PostgreSQL database port number
    pub postgresql_port: u16,
    /// PostgreSQL database name
    pub postgresql_database: String,
    /// PostgreSQL database username
    pub postgresql_username: String,
    /// PostgreSQL database password
    pub postgresql_password: String,
}
