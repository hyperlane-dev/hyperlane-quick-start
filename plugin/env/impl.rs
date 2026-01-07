use super::*;

impl EnvConfig {
    pub fn load() -> Result<Self, String> {
        let docker_config: DockerComposeConfig =
            Self::load_from_docker_compose().unwrap_or_default();
        if read_from_file::<Vec<u8>>(ENV_FILE_PATH).is_err() {
            let mut data: String = String::new();
            data.push_str(&format!("{ENV_KEY_GPT_API_URL}=\n"));
            data.push_str(&format!("{ENV_KEY_GPT_MODEL}=\n"));
            write_to_file(ENV_FILE_PATH, data.as_bytes())
                .map_err(|error| format!("Failed to create example env file: {error}"))?;
        }
        dotenvy::from_path(ENV_FILE_PATH)
            .map_err(|error| format!("Failed to load env file: {error}"))?;
        let get_env = |key: &str| -> Option<String> { std::env::var(key).ok() };
        let get_env_bool = |key: &str| -> bool {
            std::env::var(key)
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(true)
        };
        let get_env_usize = |key: &str| -> Option<usize> {
            std::env::var(key).ok().and_then(|value| value.parse().ok())
        };
        let mut config: EnvConfig = EnvConfig::default();
        config
            .set_gpt_api_url(get_env(ENV_KEY_GPT_API_URL).unwrap_or_default())
            .set_enable_mysql(true)
            .set_enable_redis(true)
            .set_enable_postgresql(true)
            .set_gtp_model(get_env(ENV_KEY_GPT_MODEL).unwrap_or_default())
            .set_mysql_host(
                get_env(ENV_KEY_MYSQL_HOST).unwrap_or_else(|| DEFAULT_DB_HOST.to_string()),
            )
            .set_mysql_port(
                docker_config
                    .get_mysql_port()
                    .or_else(|| get_env_usize(ENV_KEY_MYSQL_PORT))
                    .unwrap_or(DEFAULT_MYSQL_PORT),
            )
            .set_mysql_database(
                docker_config
                    .get_mysql_database()
                    .clone()
                    .or_else(|| get_env(ENV_KEY_MYSQL_DATABASE))
                    .unwrap_or_default(),
            )
            .set_mysql_username(
                docker_config
                    .get_mysql_username()
                    .clone()
                    .or_else(|| get_env(ENV_KEY_MYSQL_USERNAME))
                    .unwrap_or_default(),
            )
            .set_mysql_password(
                docker_config
                    .get_mysql_password()
                    .clone()
                    .or_else(|| get_env(ENV_KEY_MYSQL_PASSWORD))
                    .unwrap_or_default(),
            )
            .set_redis_host(
                get_env(ENV_KEY_REDIS_HOST).unwrap_or_else(|| DEFAULT_DB_HOST.to_string()),
            )
            .set_redis_port(
                docker_config
                    .get_redis_port()
                    .or_else(|| get_env_usize(ENV_KEY_REDIS_PORT))
                    .unwrap_or(DEFAULT_REDIS_PORT),
            )
            .set_redis_username(
                docker_config
                    .get_redis_username()
                    .clone()
                    .or_else(|| get_env(ENV_KEY_REDIS_USERNAME))
                    .unwrap_or_default(),
            )
            .set_redis_password(
                docker_config
                    .get_redis_password()
                    .clone()
                    .or_else(|| get_env(ENV_KEY_REDIS_PASSWORD))
                    .unwrap_or_default(),
            )
            .set_postgresql_host(
                get_env(ENV_KEY_POSTGRES_HOST).unwrap_or_else(|| DEFAULT_DB_HOST.to_string()),
            )
            .set_postgresql_port(
                docker_config
                    .get_postgresql_port()
                    .or_else(|| get_env_usize(ENV_KEY_POSTGRES_PORT))
                    .unwrap_or(DEFAULT_POSTGRESQL_PORT),
            )
            .set_postgresql_database(
                docker_config
                    .get_postgresql_database()
                    .clone()
                    .or_else(|| get_env(ENV_KEY_POSTGRES_DATABASE))
                    .unwrap_or_default(),
            )
            .set_postgresql_username(
                docker_config
                    .get_postgresql_username()
                    .clone()
                    .or_else(|| get_env(ENV_KEY_POSTGRES_USERNAME))
                    .unwrap_or_default(),
            )
            .set_postgresql_password(
                docker_config
                    .get_postgresql_password()
                    .clone()
                    .or_else(|| get_env(ENV_KEY_POSTGRES_PASSWORD))
                    .unwrap_or_default(),
            );

        Ok(config)
    }

    fn load_from_docker_compose() -> Result<DockerComposeConfig, String> {
        let docker_compose_content: Vec<u8> = read_from_file(DOCKER_COMPOSE_FILE_PATH)
            .map_err(|error| format!("Failed to read docker-compose.yml: {error}"))?;
        let yaml: Value = serde_yaml::from_slice(&docker_compose_content)
            .map_err(|error| format!("Failed to parse docker-compose.yml: {error}"))?;
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
