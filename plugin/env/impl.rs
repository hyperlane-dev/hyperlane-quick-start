use super::*;

impl MySqlInstanceConfig {
    pub fn get_connection_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }

    pub fn get_admin_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}

impl PostgreSqlInstanceConfig {
    pub fn get_connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }

    pub fn get_admin_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/postgres",
            self.username, self.password, self.host, self.port
        )
    }
}

impl RedisInstanceConfig {
    pub fn get_connection_url(&self) -> String {
        if self.username.is_empty() {
            format!("redis://:{}@{}:{}", self.password, self.host, self.port)
        } else {
            format!(
                "redis://{}:{}@{}:{}",
                self.username, self.password, self.host, self.port
            )
        }
    }
}

impl EnvConfig {
    pub fn get_mysql_instance(&self, name: &str) -> Option<&MySqlInstanceConfig> {
        self.mysql_instances
            .iter()
            .find(|instance| instance.name == name)
    }

    pub fn get_mysql_instance_mut(&mut self, name: &str) -> Option<&mut MySqlInstanceConfig> {
        self.mysql_instances
            .iter_mut()
            .find(|instance| instance.name == name)
    }

    pub fn add_mysql_instance(&mut self, instance: MySqlInstanceConfig) {
        self.mysql_instances.push(instance);
    }

    pub fn get_postgresql_instance(&self, name: &str) -> Option<&PostgreSqlInstanceConfig> {
        self.postgresql_instances
            .iter()
            .find(|instance| instance.name == name)
    }

    pub fn get_postgresql_instance_mut(
        &mut self,
        name: &str,
    ) -> Option<&mut PostgreSqlInstanceConfig> {
        self.postgresql_instances
            .iter_mut()
            .find(|instance| instance.name == name)
    }

    pub fn add_postgresql_instance(&mut self, instance: PostgreSqlInstanceConfig) {
        self.postgresql_instances.push(instance);
    }

    pub fn get_default_mysql_instance(&self) -> Option<&MySqlInstanceConfig> {
        self.mysql_instances.first()
    }

    pub fn get_default_postgresql_instance(&self) -> Option<&PostgreSqlInstanceConfig> {
        self.postgresql_instances.first()
    }

    pub fn get_redis_instance(&self, name: &str) -> Option<&RedisInstanceConfig> {
        self.redis_instances
            .iter()
            .find(|instance| instance.name == name)
    }

    pub fn get_redis_instance_mut(&mut self, name: &str) -> Option<&mut RedisInstanceConfig> {
        self.redis_instances
            .iter_mut()
            .find(|instance| instance.name == name)
    }

    pub fn add_redis_instance(&mut self, instance: RedisInstanceConfig) {
        self.redis_instances.push(instance);
    }

    pub fn get_default_redis_instance(&self) -> Option<&RedisInstanceConfig> {
        self.redis_instances.first()
    }

    #[instrument_trace]
    pub fn load() -> Result<Self, String> {
        let docker_config: DockerComposeConfig =
            Self::load_from_docker_compose().unwrap_or_default();
        if read_from_file::<Vec<u8>>(ENV_FILE_PATH).is_err() {
            let mut data: String = String::new();
            data.push_str(&format!("{ENV_KEY_GPT_API_URL}={BR}"));
            data.push_str(&format!("{ENV_KEY_GPT_MODEL}={BR}"));
            data.push_str(&format!(
                "{ENV_KEY_DB_CONNECTION_TIMEOUT_MILLIS}={DEFAULT_DB_CONNECTION_TIMEOUT_MILLIS}{BR}"
            ));
            write_to_file(ENV_FILE_PATH, data.as_bytes()).map_err(|error| {
                format!("Failed to create example env file{COLON_SPACE}{error}")
            })?;
        }
        dotenvy::from_path(ENV_FILE_PATH)
            .map_err(|error| format!("Failed to load env file{COLON_SPACE}{error}"))?;
        let get_env = |key: &str| -> Option<String> { std::env::var(key).ok() };
        let get_env_usize = |key: &str| -> Option<usize> {
            std::env::var(key).ok().and_then(|value| value.parse().ok())
        };
        let mut config: EnvConfig = EnvConfig {
            gpt_api_url: get_env(ENV_KEY_GPT_API_URL).unwrap_or_default(),
            gpt_model: get_env(ENV_KEY_GPT_MODEL).unwrap_or_default(),
            ..Default::default()
        };
        let default_mysql_host =
            get_env(ENV_KEY_MYSQL_HOST).unwrap_or_else(|| DEFAULT_DB_HOST.to_string());
        let default_mysql_port = docker_config
            .try_get_mysql_port()
            .or_else(|| get_env_usize(ENV_KEY_MYSQL_PORT))
            .unwrap_or(DEFAULT_MYSQL_PORT);
        let default_mysql_database = docker_config
            .try_get_mysql_database()
            .clone()
            .or_else(|| get_env(ENV_KEY_MYSQL_DATABASE))
            .unwrap_or_default();
        let default_mysql_username = docker_config
            .try_get_mysql_username()
            .clone()
            .or_else(|| get_env(ENV_KEY_MYSQL_USERNAME))
            .unwrap_or_default();
        let default_mysql_password = docker_config
            .try_get_mysql_password()
            .clone()
            .or_else(|| get_env(ENV_KEY_MYSQL_PASSWORD))
            .unwrap_or_default();
        let instance: MySqlInstanceConfig = MySqlInstanceConfig {
            name: DEFAULT_MYSQL_INSTANCE_NAME.to_string(),
            host: default_mysql_host,
            port: default_mysql_port,
            database: default_mysql_database,
            username: default_mysql_username,
            password: default_mysql_password,
        };
        config.mysql_instances.push(instance);
        let mut instance_index: usize = 1;
        loop {
            let prefix: String = format!("MYSQL_{instance_index}_");
            let host_key: String = format!("{prefix}HOST");
            if let Some(host) = get_env(&host_key) {
                let port_key: String = format!("{prefix}PORT");
                let database_key: String = format!("{prefix}DATABASE");
                let username_key: String = format!("{prefix}USERNAME");
                let password_key: String = format!("{prefix}PASSWORD");
                let instance_name: String = format!("mysql_{instance_index}");
                let instance: MySqlInstanceConfig = MySqlInstanceConfig {
                    name: instance_name,
                    host,
                    port: get_env_usize(&port_key).unwrap_or(DEFAULT_MYSQL_PORT),
                    database: get_env(&database_key).unwrap_or_default(),
                    username: get_env(&username_key).unwrap_or_default(),
                    password: get_env(&password_key).unwrap_or_default(),
                };
                config.mysql_instances.push(instance);
                instance_index += 1;
            } else {
                break;
            }
        }
        let default_postgres_host: String =
            get_env(ENV_KEY_POSTGRES_HOST).unwrap_or_else(|| DEFAULT_DB_HOST.to_string());
        let default_postgres_port: usize = docker_config
            .try_get_postgresql_port()
            .or_else(|| get_env_usize(ENV_KEY_POSTGRES_PORT))
            .unwrap_or(DEFAULT_POSTGRESQL_PORT);
        let default_postgres_database = docker_config
            .try_get_postgresql_database()
            .clone()
            .or_else(|| get_env(ENV_KEY_POSTGRES_DATABASE))
            .unwrap_or_default();
        let default_postgres_username = docker_config
            .try_get_postgresql_username()
            .clone()
            .or_else(|| get_env(ENV_KEY_POSTGRES_USERNAME))
            .unwrap_or_default();
        let default_postgres_password = docker_config
            .try_get_postgresql_password()
            .clone()
            .or_else(|| get_env(ENV_KEY_POSTGRES_PASSWORD))
            .unwrap_or_default();
        let instance: PostgreSqlInstanceConfig = PostgreSqlInstanceConfig {
            name: DEFAULT_POSTGRESQL_INSTANCE_NAME.to_string(),
            host: default_postgres_host,
            port: default_postgres_port,
            database: default_postgres_database,
            username: default_postgres_username,
            password: default_postgres_password,
        };
        config.postgresql_instances.push(instance);
        let mut instance_index: usize = 1;
        loop {
            let prefix: String = format!("POSTGRES_{instance_index}_");
            let host_key: String = format!("{prefix}HOST");
            if let Some(host) = get_env(&host_key) {
                let port_key: String = format!("{prefix}PORT");
                let database_key: String = format!("{prefix}DATABASE");
                let username_key: String = format!("{prefix}USERNAME");
                let password_key: String = format!("{prefix}PASSWORD");

                let instance_name: String = format!("postgres_{instance_index}");
                let instance = PostgreSqlInstanceConfig {
                    name: instance_name,
                    host,
                    port: get_env_usize(&port_key).unwrap_or(DEFAULT_POSTGRESQL_PORT),
                    database: get_env(&database_key).unwrap_or_default(),
                    username: get_env(&username_key).unwrap_or_default(),
                    password: get_env(&password_key).unwrap_or_default(),
                };
                config.postgresql_instances.push(instance);
                instance_index += 1;
            } else {
                break;
            }
        }
        let default_redis_host: String =
            get_env(ENV_KEY_REDIS_HOST).unwrap_or_else(|| DEFAULT_DB_HOST.to_string());
        let default_redis_port: usize = docker_config
            .try_get_redis_port()
            .or_else(|| get_env_usize(ENV_KEY_REDIS_PORT))
            .unwrap_or(DEFAULT_REDIS_PORT);
        let default_redis_username = docker_config
            .try_get_redis_username()
            .clone()
            .or_else(|| get_env(ENV_KEY_REDIS_USERNAME))
            .unwrap_or_default();
        let default_redis_password = docker_config
            .try_get_redis_password()
            .clone()
            .or_else(|| get_env(ENV_KEY_REDIS_PASSWORD))
            .unwrap_or_default();
        let instance: RedisInstanceConfig = RedisInstanceConfig {
            name: DEFAULT_REDIS_INSTANCE_NAME.to_string(),
            host: default_redis_host,
            port: default_redis_port,
            username: default_redis_username,
            password: default_redis_password,
        };
        config.redis_instances.push(instance);
        let mut instance_index: usize = 1;
        loop {
            let prefix: String = format!("REDIS_{instance_index}_");
            let host_key: String = format!("{prefix}HOST");
            if let Some(host) = get_env(&host_key) {
                let port_key: String = format!("{prefix}PORT");
                let username_key: String = format!("{prefix}USERNAME");
                let password_key: String = format!("{prefix}PASSWORD");
                let instance_name: String = format!("redis_{instance_index}");
                let instance: RedisInstanceConfig = RedisInstanceConfig {
                    name: instance_name,
                    host,
                    port: get_env_usize(&port_key).unwrap_or(DEFAULT_REDIS_PORT),
                    username: get_env(&username_key).unwrap_or_default(),
                    password: get_env(&password_key).unwrap_or_default(),
                };
                config.redis_instances.push(instance);
                instance_index += 1;
            } else {
                break;
            }
        }
        Ok(config)
    }

    #[instrument_trace]
    fn load_from_docker_compose() -> Result<DockerComposeConfig, String> {
        let docker_compose_content: Vec<u8> = read_from_file(DOCKER_COMPOSE_FILE_PATH)
            .map_err(|error| format!("Failed to read docker-compose.yml{COLON_SPACE}{error}"))?;
        let yaml: serde_yaml::Value = serde_yaml::from_slice(&docker_compose_content)
            .map_err(|error| format!("Failed to parse docker-compose.yml{COLON_SPACE}{error}"))?;
        let mut config: DockerComposeConfig = DockerComposeConfig::default();
        if let Some(mysql) = yaml
            .get(DOCKER_YAML_SERVICES)
            .and_then(|services| services.get(DOCKER_SERVICE_MYSQL))
        {
            if let Some(env) = mysql.get(DOCKER_YAML_ENVIRONMENT) {
                if let Some(database) = env
                    .get(DOCKER_MYSQL_DATABASE)
                    .and_then(|value| value.as_str())
                    .map(String::from)
                {
                    config.set_mysql_database(Some(database));
                }
                if let Some(username) = env
                    .get(DOCKER_MYSQL_USER)
                    .and_then(|value| value.as_str())
                    .map(String::from)
                {
                    config.set_mysql_username(Some(username));
                }
                if let Some(password) = env
                    .get(DOCKER_MYSQL_PASSWORD)
                    .and_then(|value| value.as_str())
                    .map(String::from)
                {
                    config.set_mysql_password(Some(password));
                }
            }
            if let Some(ports) = mysql
                .get(DOCKER_YAML_PORTS)
                .and_then(|ports_value| ports_value.as_sequence())
            {
                if let Some(port_mapping) = ports.first().and_then(|port| port.as_str()) {
                    if let Some(host_port) = port_mapping.split(':').next() {
                        if let Ok(port) = host_port.parse() {
                            config.set_mysql_port(Some(port));
                        }
                    }
                }
            }
        }
        if let Some(postgresql) = yaml
            .get(DOCKER_YAML_SERVICES)
            .and_then(|services| services.get(DOCKER_SERVICE_POSTGRESQL))
        {
            if let Some(env) = postgresql.get(DOCKER_YAML_ENVIRONMENT) {
                if let Some(database) = env
                    .get(DOCKER_POSTGRES_DB)
                    .and_then(|value| value.as_str())
                    .map(String::from)
                {
                    config.set_postgresql_database(Some(database));
                }
                if let Some(username) = env
                    .get(DOCKER_POSTGRES_USER)
                    .and_then(|value| value.as_str())
                    .map(String::from)
                {
                    config.set_postgresql_username(Some(username));
                }
                if let Some(password) = env
                    .get(DOCKER_POSTGRES_PASSWORD)
                    .and_then(|value| value.as_str())
                    .map(String::from)
                {
                    config.set_postgresql_password(Some(password));
                }
            }
            if let Some(ports) = postgresql
                .get(DOCKER_YAML_PORTS)
                .and_then(|ports_value| ports_value.as_sequence())
            {
                if let Some(port_mapping) = ports.first().and_then(|port| port.as_str()) {
                    if let Some(host_port) = port_mapping.split(':').next() {
                        if let Ok(port) = host_port.parse() {
                            config.set_postgresql_port(Some(port));
                        }
                    }
                }
            }
        }
        if let Some(redis) = yaml
            .get(DOCKER_YAML_SERVICES)
            .and_then(|services| services.get(DOCKER_SERVICE_REDIS))
        {
            if let Some(command) = redis
                .get(DOCKER_YAML_COMMAND)
                .and_then(|command_value| command_value.as_str())
            {
                if let Some(password_part) = command.split(DOCKER_REDIS_PASSWORD_FLAG).nth(1) {
                    config.set_redis_password(Some(password_part.trim().to_string()));
                }
            }
            if let Some(ports) = redis
                .get(DOCKER_YAML_PORTS)
                .and_then(|ports_value| ports_value.as_sequence())
            {
                if let Some(port_mapping) = ports.first().and_then(|port| port.as_str()) {
                    if let Some(host_port) = port_mapping.split(':').next() {
                        if let Ok(port) = host_port.parse() {
                            config.set_redis_port(Some(port));
                        }
                    }
                }
            }
        }
        Ok(config)
    }
}
