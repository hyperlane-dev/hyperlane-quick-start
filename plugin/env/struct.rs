use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct EnvPlugin;

#[derive(Clone, Data, Debug, Default)]
pub struct DockerComposeConfig {
    pub(super) mysql_database: Option<String>,
    pub(super) mysql_password: Option<String>,
    #[get(type(copy))]
    pub(super) mysql_port: Option<usize>,
    pub(super) mysql_username: Option<String>,
    pub(super) postgresql_database: Option<String>,
    pub(super) postgresql_password: Option<String>,
    #[get(type(copy))]
    pub(super) postgresql_port: Option<usize>,
    pub(super) postgresql_username: Option<String>,
    pub(super) redis_password: Option<String>,
    #[get(type(copy))]
    pub(super) redis_port: Option<usize>,
    pub(super) redis_username: Option<String>,
}

#[derive(Clone, Data, Debug, Default)]
pub struct EnvConfig {
    #[get(type(copy))]
    pub(super) db_connection_timeout_millis: u64,
    #[get(type(copy))]
    pub(super) db_retry_interval_millis: u64,
    #[get(pub)]
    pub(super) gpt_api_url: String,
    #[get(pub)]
    pub(super) gpt_model: String,
    pub(super) mysql_instances: Vec<MySqlInstanceConfig>,
    pub(super) postgresql_instances: Vec<PostgreSqlInstanceConfig>,
    pub(super) redis_instances: Vec<RedisInstanceConfig>,
    #[get(type(copy))]
    pub(super) server_port: u16,
    #[get(pub)]
    pub(super) server_host: String,
    #[get(type(copy))]
    pub(super) server_buffer: usize,
    #[get(type(copy))]
    pub(super) server_log_size: usize,
    #[get(pub)]
    pub(super) server_log_dir: String,
    #[get(type(copy))]
    pub(super) server_inner_print: bool,
    #[get(type(copy))]
    pub(super) server_inner_log: bool,
    #[get(type(copy))]
    pub(super) server_nodelay: Option<bool>,
    #[get(type(copy))]
    pub(super) server_tti: Option<u32>,
    #[get(pub)]
    pub(super) server_pid_file_path: String,
    #[get(type(copy))]
    pub(super) server_request_http_read_timeout_ms: u64,
    #[get(type(copy))]
    pub(super) server_request_max_body_size: usize,
}

#[derive(Clone, Debug, Default, serde::Deserialize, Data)]
pub struct MySqlInstanceConfig {
    #[serde(rename = "name")]
    pub(super) name: String,
    #[serde(rename = "host")]
    pub(super) host: String,
    #[get(type(copy))]
    #[serde(default, rename = "port")]
    pub(super) port: usize,
    #[serde(rename = "database")]
    pub(super) database: String,
    #[serde(rename = "username")]
    pub(super) username: String,
    #[serde(rename = "password")]
    pub(super) password: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, Data)]
pub struct PostgreSqlInstanceConfig {
    #[serde(rename = "name")]
    pub(super) name: String,
    #[serde(rename = "host")]
    pub(super) host: String,
    #[get(type(copy))]
    #[serde(default, rename = "port")]
    pub(super) port: usize,
    #[serde(rename = "database")]
    pub(super) database: String,
    #[serde(rename = "username")]
    pub(super) username: String,
    #[serde(rename = "password")]
    pub(super) password: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, Data)]
pub struct RedisInstanceConfig {
    #[serde(rename = "name")]
    pub(super) name: String,
    #[serde(rename = "host")]
    pub(super) host: String,
    #[get(type(copy))]
    #[serde(default, rename = "port")]
    pub(super) port: usize,
    #[serde(default, rename = "username")]
    pub(super) username: String,
    #[serde(rename = "password")]
    pub(super) password: String,
}
