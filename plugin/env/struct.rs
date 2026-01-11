use super::*;

#[derive(Clone, Data, Debug, Default)]
pub struct EnvConfig {
    gpt_api_url: String,
    gpt_model: String,
    mysql_host: String,
    mysql_port: usize,
    mysql_database: String,
    mysql_username: String,
    mysql_password: String,
    redis_host: String,
    redis_port: usize,
    redis_username: String,
    redis_password: String,
    postgresql_host: String,
    postgresql_port: usize,
    postgresql_database: String,
    postgresql_username: String,
    postgresql_password: String,
}

#[derive(Clone, Data, Debug, Default)]
pub struct DockerComposeConfig {
    mysql_port: Option<usize>,
    mysql_database: Option<String>,
    mysql_username: Option<String>,
    mysql_password: Option<String>,
    redis_port: Option<usize>,
    redis_username: Option<String>,
    redis_password: Option<String>,
    postgresql_port: Option<usize>,
    postgresql_database: Option<String>,
    postgresql_username: Option<String>,
    postgresql_password: Option<String>,
}
