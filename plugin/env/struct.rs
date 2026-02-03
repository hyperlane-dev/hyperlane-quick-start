use super::*;

#[derive(Clone, Data, Debug, Default)]
pub struct DockerComposeConfig {
    #[get(pub(crate))]
    pub(super) mysql_database: Option<String>,
    #[get(pub(crate))]
    pub(super) mysql_password: Option<String>,
    #[get(type(copy), pub(crate))]
    pub(super) mysql_port: Option<usize>,
    #[get(pub(crate))]
    pub(super) mysql_username: Option<String>,
    #[get(pub(crate))]
    pub(super) postgresql_database: Option<String>,
    #[get(pub(crate))]
    pub(super) postgresql_password: Option<String>,
    #[get(type(copy), pub(crate))]
    pub(super) postgresql_port: Option<usize>,
    #[get(pub(crate))]
    pub(super) postgresql_username: Option<String>,
    #[get(pub(crate))]
    pub(super) redis_password: Option<String>,
    #[get(type(copy), pub(crate))]
    pub(super) redis_port: Option<usize>,
    #[get(pub(crate))]
    pub(super) redis_username: Option<String>,
}

#[derive(Clone, Data, Debug, Default)]
pub struct EnvConfig {
    #[get(pub)]
    pub(super) gpt_api_url: String,
    #[get(pub)]
    pub(super) gpt_model: String,
    #[get(pub(crate))]
    pub(super) mysql_instances: Vec<MySqlInstanceConfig>,
    #[get(pub(crate))]
    pub(super) postgresql_instances: Vec<PostgreSqlInstanceConfig>,
    #[get(pub(crate))]
    pub(super) redis_instances: Vec<RedisInstanceConfig>,
}

#[derive(Clone, Data, Debug, Default)]
pub struct MySqlInstanceConfig {
    #[get(pub(crate))]
    pub(super) database: String,
    #[get(pub(crate))]
    pub(super) host: String,
    #[get(pub(crate))]
    pub(super) name: String,
    #[get(pub(crate))]
    pub(super) password: String,
    #[get(type(copy), pub(crate))]
    pub(super) port: usize,
    #[get(pub(crate))]
    pub(super) username: String,
}

#[derive(Clone, Data, Debug, Default)]
pub struct PostgreSqlInstanceConfig {
    #[get(pub(crate))]
    pub(super) database: String,
    #[get(pub(crate))]
    pub(super) host: String,
    #[get(pub(crate))]
    pub(super) name: String,
    #[get(pub(crate))]
    pub(super) password: String,
    #[get(type(copy), pub(crate))]
    pub(super) port: usize,
    #[get(pub(crate))]
    pub(super) username: String,
}

#[derive(Clone, Data, Debug, Default)]
pub struct RedisInstanceConfig {
    #[get(pub(crate))]
    pub(super) host: String,
    #[get(pub(crate))]
    pub(super) name: String,
    #[get(pub(crate))]
    pub(super) password: String,
    #[get(type(copy), pub(crate))]
    pub(super) port: usize,
    #[get(pub(crate))]
    pub(super) username: String,
}
