use super::*;

#[derive(Clone, Data, Debug, Default)]
pub struct MySqlInstanceConfig {
    pub name: String,
    pub host: String,
    pub port: usize,
    pub database: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Data, Debug, Default)]
pub struct PostgreSqlInstanceConfig {
    pub name: String,
    pub host: String,
    pub port: usize,
    pub database: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Data, Debug, Default)]
pub struct EnvConfig {
    pub gpt_api_url: String,
    pub gpt_model: String,
    pub mysql_instances: Vec<MySqlInstanceConfig>,
    pub redis_host: String,
    pub redis_port: usize,
    pub redis_username: String,
    pub redis_password: String,
    pub postgresql_instances: Vec<PostgreSqlInstanceConfig>,
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
