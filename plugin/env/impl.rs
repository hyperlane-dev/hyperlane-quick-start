use super::*;

impl GetOrInit for EnvPlugin {
    type Instance = EnvConfig;

    #[instrument_trace]
    fn get_or_init() -> &'static Self::Instance {
        GLOBAL_ENV_CONFIG.get_or_init(EnvConfig::default)
    }
}

impl EnvPlugin {
    #[instrument_trace]
    pub fn try_load_config() -> Result<(), String> {
        let config: EnvConfig = EnvConfig::load()?;
        GLOBAL_ENV_CONFIG
            .set(config.clone())
            .map_err(|_| "Failed to initialize global environment configuration".to_string())?;
        Ok(())
    }
}

impl MySqlInstanceConfig {
    pub(crate) fn get_connection_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.get_username(),
            self.get_password(),
            self.get_host(),
            self.get_port(),
            self.get_database()
        )
    }
    pub(crate) fn get_admin_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}",
            self.get_username(),
            self.get_password(),
            self.get_host(),
            self.get_port()
        )
    }
}

impl PostgreSqlInstanceConfig {
    pub(crate) fn get_connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.get_username(),
            self.get_password(),
            self.get_host(),
            self.get_port(),
            self.get_database()
        )
    }
    pub(crate) fn get_admin_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/postgres",
            self.get_username(),
            self.get_password(),
            self.get_host(),
            self.get_port()
        )
    }
}

impl RedisInstanceConfig {
    pub(crate) fn get_connection_url(&self) -> String {
        let port: usize = self.get_port();
        if self.get_username().is_empty() {
            format!(
                "redis://:{}@{}:{}",
                self.get_password(),
                self.get_host(),
                port
            )
        } else {
            format!(
                "redis://{}:{}@{}:{}",
                self.get_username(),
                self.get_password(),
                self.get_host(),
                port
            )
        }
    }
}

impl EnvConfig {
    pub(crate) fn get_mysql_instance(&self, name: &str) -> Option<&MySqlInstanceConfig> {
        self.get_mysql_instances()
            .iter()
            .find(|instance| instance.get_name() == name)
    }
    pub(crate) fn get_postgresql_instance(&self, name: &str) -> Option<&PostgreSqlInstanceConfig> {
        self.get_postgresql_instances()
            .iter()
            .find(|instance| instance.get_name() == name)
    }
    pub(crate) fn get_default_mysql_instance(&self) -> Option<&MySqlInstanceConfig> {
        self.get_mysql_instances().first()
    }
    pub(crate) fn get_default_postgresql_instance(&self) -> Option<&PostgreSqlInstanceConfig> {
        self.get_postgresql_instances().first()
    }
    pub(crate) fn get_redis_instance(&self, name: &str) -> Option<&RedisInstanceConfig> {
        self.get_redis_instances()
            .iter()
            .find(|instance| instance.get_name() == name)
    }
    pub(crate) fn get_default_redis_instance(&self) -> Option<&RedisInstanceConfig> {
        self.get_redis_instances().first()
    }

    #[instrument_trace]
    pub(crate) fn load() -> Result<Self, String> {
        dotenvy::from_path(SERVER_ENV_FILE_PATH)
            .map_err(|error: dotenvy::Error| format!("Failed to load env file {error}"))?;
        let get_env_required = |key: &str| -> Result<String, String> {
            var(key).map_err(|_| format!("Environment variable {} is not set", key))
        };
        let get_env_u16 = |key: &str| -> Result<u16, String> {
            var(key)
                .map_err(|_| format!("Environment variable {} is not set", key))?
                .parse()
                .map_err(|_| format!("Environment variable {} must be a valid u16", key))
        };
        let get_env_u32 = |key: &str| -> Result<u32, String> {
            var(key)
                .map_err(|_| format!("Environment variable {} is not set", key))?
                .parse()
                .map_err(|_| format!("Environment variable {} must be a valid u32", key))
        };
        let get_env_u64 = |key: &str| -> Result<u64, String> {
            var(key)
                .map_err(|_| format!("Environment variable {} is not set", key))?
                .parse()
                .map_err(|_| format!("Environment variable {} must be a valid u64", key))
        };
        let get_env_usize = |key: &str| -> Result<usize, String> {
            var(key)
                .map_err(|_| format!("Environment variable {} is not set", key))?
                .parse()
                .map_err(|_| format!("Environment variable {} must be a valid usize", key))
        };
        let get_env_bool = |key: &str| -> Result<bool, String> {
            let value = var(key).map_err(|_| format!("Environment variable {} is not set", key))?;
            if value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("1") {
                Ok(true)
            } else if value.eq_ignore_ascii_case("false") || value.eq_ignore_ascii_case("0") {
                Ok(false)
            } else {
                Err(format!(
                    "Environment variable {} must be true/false or 1/0",
                    key
                ))
            }
        };
        let docker_config: DockerComposeConfig =
            Self::load_from_docker_compose(SERVER_DOCKER_COMPOSE_FILE_PATH).unwrap_or_default();
        let mysql_instances: Vec<MySqlInstanceConfig> =
            Self::parse_mysql_instances(&docker_config)?;
        let postgresql_instances: Vec<PostgreSqlInstanceConfig> =
            Self::parse_postgresql_instances(&docker_config)?;
        let redis_instances: Vec<RedisInstanceConfig> =
            Self::parse_redis_instances(&docker_config)?;
        let config: EnvConfig = EnvConfig {
            db_connection_timeout_millis: get_env_u64(ENV_KEY_DB_CONNECTION_TIMEOUT_MILLIS)?,
            db_retry_interval_millis: get_env_u64(ENV_KEY_DB_RETRY_INTERVAL_MILLIS)?,
            gpt_api_url: var(ENV_KEY_GPT_API_URL).unwrap_or_default(),
            gpt_api_key: var(ENV_KEY_GPT_API_KEY).unwrap_or_default(),
            gpt_model: var(ENV_KEY_GPT_MODEL).unwrap_or_default(),
            mysql_instances,
            postgresql_instances,
            redis_instances,
            server_port: get_env_u16(ENV_KEY_SERVER_PORT)?,
            server_host: get_env_required(ENV_KEY_SERVER_HOST)?,
            server_buffer: get_env_usize(ENV_KEY_SERVER_BUFFER)?,
            server_log_size: get_env_usize(ENV_KEY_SERVER_LOG_SIZE)?,
            server_log_dir: get_env_required(ENV_KEY_SERVER_LOG_DIR)?,
            server_inner_print: get_env_bool(ENV_KEY_SERVER_INNER_PRINT)?,
            server_inner_log: get_env_bool(ENV_KEY_SERVER_INNER_LOG)?,
            server_nodelay: Some(get_env_bool(ENV_KEY_SERVER_NODELAY)?),
            server_tti: Some(get_env_u32(ENV_KEY_SERVER_TTI)?),
            server_pid_file_path: get_env_required(ENV_KEY_SERVER_PID_FILE_PATH)?,
            server_request_http_read_timeout_ms: get_env_u64(
                ENV_KEY_SERVER_REQUEST_HTTP_READ_TIMEOUT_MS,
            )?,
            server_request_max_body_size: get_env_usize(ENV_KEY_SERVER_REQUEST_MAX_BODY_SIZE)?,
        };
        Ok(config)
    }

    fn parse_mysql_instances(
        docker_config: &DockerComposeConfig,
    ) -> Result<Vec<MySqlInstanceConfig>, String> {
        let mysql_json: String = var(ENV_KEY_MYSQL)
            .map_err(|_| format!("Environment variable {} is not set", ENV_KEY_MYSQL))?
            .trim_matches('\'')
            .to_string();
        let mut instances: Vec<MySqlInstanceConfig> = serde_json::from_str(&mysql_json)
            .map_err(|error: Error| format!("Failed to parse {}: {}", ENV_KEY_MYSQL, error))?;
        for instance in instances.iter_mut() {
            if instance.get_port() == 0 {
                instance.set_port(docker_config.get_mysql_port().unwrap_or(3306));
            }
        }
        Ok(instances)
    }

    fn parse_postgresql_instances(
        docker_config: &DockerComposeConfig,
    ) -> Result<Vec<PostgreSqlInstanceConfig>, String> {
        let postgresql_json: String = var(ENV_KEY_POSTGRESQL)
            .map_err(|_| format!("Environment variable {} is not set", ENV_KEY_POSTGRESQL))?
            .trim_matches('\'')
            .to_string();
        let mut instances: Vec<PostgreSqlInstanceConfig> = serde_json::from_str(&postgresql_json)
            .map_err(|error: Error| {
            format!("Failed to parse {}: {}", ENV_KEY_POSTGRESQL, error)
        })?;
        for instance in instances.iter_mut() {
            if instance.get_port() == 0 {
                instance.set_port(docker_config.get_postgresql_port().unwrap_or(5432));
            }
        }
        Ok(instances)
    }

    fn parse_redis_instances(
        docker_config: &DockerComposeConfig,
    ) -> Result<Vec<RedisInstanceConfig>, String> {
        let redis_json: String = var(ENV_KEY_REDIS)
            .map_err(|_| format!("Environment variable {} is not set", ENV_KEY_REDIS))?
            .trim_matches('\'')
            .to_string();
        let mut instances: Vec<RedisInstanceConfig> = serde_json::from_str(&redis_json)
            .map_err(|error: Error| format!("Failed to parse {}: {}", ENV_KEY_REDIS, error))?;
        for instance in instances.iter_mut() {
            if instance.get_port() == 0 {
                instance.set_port(docker_config.get_redis_port().unwrap_or(6379));
            }
        }
        Ok(instances)
    }

    #[instrument_trace]
    fn load_from_docker_compose(file_path: &str) -> Result<DockerComposeConfig, String> {
        let docker_compose_content: Vec<u8> =
            read_from_file(file_path).map_err(|error: Box<dyn std::error::Error>| {
                format!("Failed to read docker-compose.yml {error}")
            })?;
        let yaml: serde_yaml::Value = serde_yaml::from_slice(&docker_compose_content).map_err(
            |error: serde_yaml::Error| format!("Failed to parse docker-compose.yml {error}"),
        )?;
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
                && let Some(port_mapping) = ports.first().and_then(|port| port.as_str())
                && let Some(host_port) = port_mapping.split(':').next()
                && let Ok(port) = host_port.parse()
            {
                config.set_mysql_port(Some(port));
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
                && let Some(port_mapping) = ports.first().and_then(|port| port.as_str())
                && let Some(host_port) = port_mapping.split(':').next()
                && let Ok(port) = host_port.parse()
            {
                config.set_postgresql_port(Some(port));
            }
        }
        if let Some(redis) = yaml
            .get(DOCKER_YAML_SERVICES)
            .and_then(|services| services.get(DOCKER_SERVICE_REDIS))
        {
            if let Some(command) = redis
                .get(DOCKER_YAML_COMMAND)
                .and_then(|command_value| command_value.as_str())
                && let Some(password_part) = command.split(DOCKER_REDIS_PASSWORD_FLAG).nth(1)
            {
                config.set_redis_password(Some(password_part.trim().to_string()));
            }
            if let Some(ports) = redis
                .get(DOCKER_YAML_PORTS)
                .and_then(|ports_value| ports_value.as_sequence())
                && let Some(port_mapping) = ports.first().and_then(|port| port.as_str())
                && let Some(host_port) = port_mapping.split(':').next()
                && let Ok(port) = host_port.parse()
            {
                config.set_redis_port(Some(port));
            }
        }
        Ok(config)
    }

    #[instrument_trace]
    pub fn log_config() {
        #[cfg(debug_assertions)]
        let is_dev: bool = true;
        #[cfg(not(debug_assertions))]
        let is_dev: bool = false;
        let config: &EnvConfig = EnvPlugin::get_or_init();
        if is_dev {
            info!("Environment Configuration Loaded Successfully");
            info!("Database Configuration:");
            info!(
                "  DB_CONNECTION_TIMEOUT_MILLIS: {}",
                config.get_db_connection_timeout_millis()
            );
            info!(
                "  DB_RETRY_INTERVAL_MILLIS: {}",
                config.get_db_retry_interval_millis()
            );
            info!("GPT Configuration:");
            info!(
                "  GPT_API_URL: {}",
                if config.get_gpt_api_url().is_empty() {
                    "(not set)"
                } else {
                    config.get_gpt_api_url()
                }
            );
            info!(
                "  GPT_MODEL: {}",
                if config.get_gpt_model().is_empty() {
                    "(not set)"
                } else {
                    config.get_gpt_model()
                }
            );
            info!("MySQL Configuration:");
            if config.get_mysql_instances().is_empty() {
                info!("  (no MySQL instances configured)");
            } else {
                for instance in config.get_mysql_instances() {
                    info!("  Instance '{}'", instance.get_name());
                    info!("    Host: {}", instance.get_host());
                    info!("    Port: {}", instance.get_port());
                    info!("    Database: {}", instance.get_database());
                    info!("    Username: {}", instance.get_username());
                    info!("    Password: {}", instance.get_password());
                }
            }
            info!("PostgreSQL Configuration:");
            if config.get_postgresql_instances().is_empty() {
                info!("  (no PostgreSQL instances configured)");
            } else {
                for instance in config.get_postgresql_instances() {
                    info!("  Instance '{}'", instance.get_name());
                    info!("    Host: {}", instance.get_host());
                    info!("    Port: {}", instance.get_port());
                    info!("    Database: {}", instance.get_database());
                    info!("    Username: {}", instance.get_username());
                    info!("    Password: {}", instance.get_password());
                }
            }
            info!("Redis Configuration:");
            if config.get_redis_instances().is_empty() {
                info!("  (no Redis instances configured)");
            } else {
                for instance in config.get_redis_instances() {
                    info!("  Instance '{}'", instance.get_name());
                    info!("    Host: {}", instance.get_host());
                    info!("    Port: {}", instance.get_port());
                    info!(
                        "    Username: {}",
                        if instance.get_username().is_empty() {
                            "(none)"
                        } else {
                            instance.get_username()
                        }
                    );
                    info!("    Password: {}", instance.get_password());
                }
            }
            info!("Server Configuration:");
            info!("  SERVER_PORT: {}", config.get_server_port());
            info!("  SERVER_HOST: {}", config.get_server_host());
            info!("  SERVER_BUFFER: {}", config.get_server_buffer());
            info!("  SERVER_LOG_SIZE: {}", config.get_server_log_size());
            info!("  SERVER_LOG_DIR: {}", config.get_server_log_dir());
            info!("  SERVER_INNER_PRINT: {}", config.get_server_inner_print());
            info!("  SERVER_INNER_LOG: {}", config.get_server_inner_log());
            info!("  SERVER_NODELAY: {:?}", config.get_server_nodelay());
            info!("  SERVER_TTI: {:?}", config.get_server_tti());
            info!(
                "  SERVER_PID_FILE_PATH: {}",
                config.get_server_pid_file_path()
            );
            info!(
                "  SERVER_REQUEST_HTTP_READ_TIMEOUT_MS: {}",
                config.get_server_request_http_read_timeout_ms()
            );
            info!(
                "  SERVER_REQUEST_MAX_BODY_SIZE: {}",
                config.get_server_request_max_body_size()
            );
        } else {
            info!(
                "GPT API URL {}",
                if config.get_gpt_api_url().is_empty() {
                    "(not set)"
                } else {
                    config.get_gpt_api_url()
                }
            );
            info!(
                "GPT Model {}",
                if config.get_gpt_model().is_empty() {
                    "(not set)"
                } else {
                    config.get_gpt_model()
                }
            );
            info!("MySQL Configuration:");
            if config.get_mysql_instances().is_empty() {
                info!("  (no MySQL instances configured)");
            } else {
                for instance in config.get_mysql_instances() {
                    info!(
                        "  Instance '{}' {}:{}@{}:{}/{}",
                        instance.get_name(),
                        instance.get_username(),
                        "***",
                        instance.get_host(),
                        instance.get_port(),
                        instance.get_database()
                    );
                }
            }
            info!("PostgreSQL Configuration:");
            if config.get_postgresql_instances().is_empty() {
                info!("  (no PostgreSQL instances configured)");
            } else {
                for instance in config.get_postgresql_instances() {
                    info!(
                        "  Instance '{}' {}:{}@{}:{}/{}",
                        instance.get_name(),
                        instance.get_username(),
                        "***",
                        instance.get_host(),
                        instance.get_port(),
                        instance.get_database()
                    );
                }
            }
            info!("Redis Configuration:");
            if config.get_redis_instances().is_empty() {
                info!("  (no Redis instances configured)");
            } else {
                for instance in config.get_redis_instances() {
                    info!(
                        "  Instance '{}' {}:{}@{}:{}",
                        instance.get_name(),
                        if instance.get_username().is_empty() {
                            "(none)"
                        } else {
                            instance.get_username()
                        },
                        "***",
                        instance.get_host(),
                        instance.get_port()
                    );
                }
            }
        }
    }
}
