use super::*;

/// Implementation of `GetOrInit` for `EnvPlugin`, providing lazy initialization of the global `EnvConfig`.
impl GetOrInit for EnvPlugin {
    type Instance = EnvConfig;

    /// Lazily initializes and returns a static reference to the global `EnvConfig` singleton.
    ///
    /// # Returns
    ///
    /// - `&'static EnvConfig`: The static reference to the global environment configuration.
    #[instrument_trace]
    fn get_or_init() -> &'static Self::Instance {
        GLOBAL_ENV_CONFIG.get_or_init(EnvConfig::default)
    }
}

/// Implementation of configuration loading methods for `EnvPlugin`.
impl EnvPlugin {
    /// Attempts to load the environment configuration from the env file and store it globally.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok if the configuration was loaded successfully, or an error message on failure.
    #[instrument_trace]
    pub fn try_load_config() -> Result<(), String> {
        let config: EnvConfig = EnvConfig::load()?;
        GLOBAL_ENV_CONFIG
            .set(config.clone())
            .map_err(|_: EnvConfig| {
                "Failed to initialize global environment configuration".to_string()
            })?;
        Ok(())
    }
}

/// Implementation of connection URL generation methods for `MySqlInstanceConfig`.
impl MySqlInstanceConfig {
    /// Returns the MySQL connection URL for this instance, including the database name.
    ///
    /// # Returns
    ///
    /// - `String`: The MySQL connection URL in the format `mysql://user:password@host:port/database`.
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

    /// Returns the MySQL admin connection URL for this instance, without a specific database.
    ///
    /// # Returns
    ///
    /// - `String`: The MySQL admin URL in the format `mysql://user:password@host:port`.
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

/// Implementation of connection URL generation methods for `PostgreSqlInstanceConfig`.
impl PostgreSqlInstanceConfig {
    /// Returns the PostgreSQL connection URL for this instance, including the database name.
    ///
    /// # Returns
    ///
    /// - `String`: The PostgreSQL connection URL in the format `postgres://user:password@host:port/database`.
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

    /// Returns the PostgreSQL admin connection URL for this instance, connecting to the default `postgres` database.
    ///
    /// # Returns
    ///
    /// - `String`: The PostgreSQL admin URL in the format `postgres://user:password@host:port/postgres`.
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

/// Implementation of connection URL generation methods for `RedisInstanceConfig`.
impl RedisInstanceConfig {
    /// Returns the Redis connection URL for this instance, with or without a username.
    ///
    /// # Returns
    ///
    /// - `String`: The Redis connection URL in the format `redis://user:password@host:port` or `redis://:password@host:port`.
    pub(crate) fn get_connection_url(&self) -> String {
        if self.get_username().is_empty() {
            format!(
                "redis://:{}@{}:{}",
                self.get_password(),
                self.get_host(),
                self.get_port()
            )
        } else {
            format!(
                "redis://{}:{}@{}:{}",
                self.get_username(),
                self.get_password(),
                self.get_host(),
                self.get_port()
            )
        }
    }
}

/// Implementation of instance lookup, configuration loading, and logging methods for `EnvConfig`.
impl EnvConfig {
    /// Returns the MySQL instance configuration with the specified name.
    ///
    /// # Arguments
    ///
    /// - `&str`: The name of the MySQL instance to find.
    ///
    /// # Returns
    ///
    /// - `Option<&MySqlInstanceConfig>`: The instance configuration if found, or None.
    pub(crate) fn get_mysql_instance(&self, name: &str) -> Option<&MySqlInstanceConfig> {
        self.get_mysql_instances()
            .iter()
            .find(|instance: &&MySqlInstanceConfig| instance.get_name() == name)
    }

    /// Returns the PostgreSQL instance configuration with the specified name.
    ///
    /// # Arguments
    ///
    /// - `&str`: The name of the PostgreSQL instance to find.
    ///
    /// # Returns
    ///
    /// - `Option<&PostgreSqlInstanceConfig>`: The instance configuration if found, or None.
    pub(crate) fn get_postgresql_instance(&self, name: &str) -> Option<&PostgreSqlInstanceConfig> {
        self.get_postgresql_instances()
            .iter()
            .find(|instance: &&PostgreSqlInstanceConfig| instance.get_name() == name)
    }

    /// Returns the first MySQL instance configuration as the default instance.
    ///
    /// # Returns
    ///
    /// - `Option<&MySqlInstanceConfig>`: The default MySQL instance if any exist, or None.
    pub(crate) fn get_default_mysql_instance(&self) -> Option<&MySqlInstanceConfig> {
        self.get_mysql_instances().first()
    }

    /// Returns the first PostgreSQL instance configuration as the default instance.
    ///
    /// # Returns
    ///
    /// - `Option<&PostgreSqlInstanceConfig>`: The default PostgreSQL instance if any exist, or None.
    pub(crate) fn get_default_postgresql_instance(&self) -> Option<&PostgreSqlInstanceConfig> {
        self.get_postgresql_instances().first()
    }

    /// Returns the Redis instance configuration with the specified name.
    ///
    /// # Arguments
    ///
    /// - `&str`: The name of the Redis instance to find.
    ///
    /// # Returns
    ///
    /// - `Option<&RedisInstanceConfig>`: The instance configuration if found, or None.
    pub(crate) fn get_redis_instance(&self, name: &str) -> Option<&RedisInstanceConfig> {
        self.get_redis_instances()
            .iter()
            .find(|instance: &&RedisInstanceConfig| instance.get_name() == name)
    }

    /// Returns the first Redis instance configuration as the default instance.
    ///
    /// # Returns
    ///
    /// - `Option<&RedisInstanceConfig>`: The default Redis instance if any exist, or None.
    pub(crate) fn get_default_redis_instance(&self) -> Option<&RedisInstanceConfig> {
        self.get_redis_instances().first()
    }

    /// Loads the environment configuration from the env file and Docker Compose configuration.
    ///
    /// # Returns
    ///
    /// - `Result<Self, String>`: The loaded configuration on success, or an error message on failure.
    #[instrument_trace]
    pub(crate) fn load() -> Result<Self, String> {
        dotenvy::from_path(SERVER_ENV_FILE_PATH)
            .map_err(|error: dotenvy::Error| format!("Failed to load env file {error}"))?;
        let get_env_required = |key: &str| -> Result<String, String> {
            var(key).map_err(|_: VarError| format!("Environment variable {key} is not set"))
        };
        let get_env_u16 = |key: &str| -> Result<u16, String> {
            var(key)
                .map_err(|_: VarError| format!("Environment variable {key} is not set"))?
                .parse::<u16>()
                .map_err(|_: ParseIntError| {
                    format!("Environment variable {key} must be a valid u16")
                })
        };
        let get_env_u32 = |key: &str| -> Result<u32, String> {
            var(key)
                .map_err(|_: VarError| format!("Environment variable {key} is not set"))?
                .parse::<u32>()
                .map_err(|_: ParseIntError| {
                    format!("Environment variable {key} must be a valid u32")
                })
        };
        let get_env_u64 = |key: &str| -> Result<u64, String> {
            var(key)
                .map_err(|_: VarError| format!("Environment variable {key} is not set"))?
                .parse::<u64>()
                .map_err(|_: ParseIntError| {
                    format!("Environment variable {key} must be a valid u64")
                })
        };
        let get_env_usize = |key: &str| -> Result<usize, String> {
            var(key)
                .map_err(|_: VarError| format!("Environment variable {key} is not set"))?
                .parse::<usize>()
                .map_err(|_: ParseIntError| {
                    format!("Environment variable {key} must be a valid usize")
                })
        };
        let get_env_bool = |key: &str| -> Result<bool, String> {
            let value: String =
                var(key).map_err(|_: VarError| format!("Environment variable {key} is not set"))?;
            if value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("1") {
                Ok(true)
            } else if value.eq_ignore_ascii_case("false") || value.eq_ignore_ascii_case("0") {
                Ok(false)
            } else {
                Err(format!(
                    "Environment variable {key} must be true/false or 1/0"
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
            gpt_enable_thinking: get_env_bool(ENV_KEY_GPT_ENABLE_THINKING)?,
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

    /// Parses MySQL instance configurations from the environment variable and merges with Docker Compose defaults.
    ///
    /// # Arguments
    ///
    /// - `&DockerComposeConfig`: The Docker Compose configuration providing default port values.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<MySqlInstanceConfig>, String>`: The parsed MySQL instances on success, or an error message on failure.
    fn parse_mysql_instances(
        docker_config: &DockerComposeConfig,
    ) -> Result<Vec<MySqlInstanceConfig>, String> {
        let mut instances: Vec<MySqlInstanceConfig> = serde_json::from_str(
            var(ENV_KEY_MYSQL)
                .map_err(|_: VarError| format!("Environment variable {ENV_KEY_MYSQL} is not set"))?
                .trim_matches('\''),
        )
        .map_err(|error: serde_json::Error| format!("Failed to parse {ENV_KEY_MYSQL}: {error}"))?;
        instances
            .iter_mut()
            .for_each(|instance: &mut MySqlInstanceConfig| {
                if instance.get_port() == 0 {
                    instance.set_port(docker_config.get_mysql_port().unwrap_or(3306));
                }
            });
        Ok(instances)
    }

    /// Parses PostgreSQL instance configurations from the environment variable and merges with Docker Compose defaults.
    ///
    /// # Arguments
    ///
    /// - `&DockerComposeConfig`: The Docker Compose configuration providing default port values.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<PostgreSqlInstanceConfig>, String>`: The parsed PostgreSQL instances on success, or an error message on failure.
    fn parse_postgresql_instances(
        docker_config: &DockerComposeConfig,
    ) -> Result<Vec<PostgreSqlInstanceConfig>, String> {
        let mut instances: Vec<PostgreSqlInstanceConfig> = serde_json::from_str(
            var(ENV_KEY_POSTGRESQL)
                .map_err(|_: VarError| {
                    format!("Environment variable {ENV_KEY_POSTGRESQL} is not set")
                })?
                .trim_matches('\''),
        )
        .map_err(|error: serde_json::Error| {
            format!("Failed to parse {ENV_KEY_POSTGRESQL}: {error}")
        })?;
        instances
            .iter_mut()
            .for_each(|instance: &mut PostgreSqlInstanceConfig| {
                if instance.get_port() == 0 {
                    instance.set_port(docker_config.get_postgresql_port().unwrap_or(5432));
                }
            });
        Ok(instances)
    }

    /// Parses Redis instance configurations from the environment variable and merges with Docker Compose defaults.
    ///
    /// # Arguments
    ///
    /// - `&DockerComposeConfig`: The Docker Compose configuration providing default port values.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<RedisInstanceConfig>, String>`: The parsed Redis instances on success, or an error message on failure.
    fn parse_redis_instances(
        docker_config: &DockerComposeConfig,
    ) -> Result<Vec<RedisInstanceConfig>, String> {
        let mut instances: Vec<RedisInstanceConfig> = serde_json::from_str(
            var(ENV_KEY_REDIS)
                .map_err(|_: VarError| format!("Environment variable {ENV_KEY_REDIS} is not set"))?
                .trim_matches('\''),
        )
        .map_err(|error: serde_json::Error| format!("Failed to parse {ENV_KEY_REDIS}: {error}"))?;
        instances
            .iter_mut()
            .for_each(|instance: &mut RedisInstanceConfig| {
                if instance.get_port() == 0 {
                    instance.set_port(docker_config.get_redis_port().unwrap_or(6379));
                }
            });
        Ok(instances)
    }

    /// Loads the Docker Compose configuration from the specified file path and extracts service connection details.
    ///
    /// # Arguments
    ///
    /// - `&str`: The file path to the Docker Compose YAML file.
    ///
    /// # Returns
    ///
    /// - `Result<DockerComposeConfig, String>`: The parsed Docker Compose configuration on success, or an error message on failure.
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
            .and_then(|services: &serde_yaml::Value| services.get(DOCKER_SERVICE_MYSQL))
        {
            if let Some(env) = mysql.get(DOCKER_YAML_ENVIRONMENT) {
                if let Some(database) = env
                    .get(DOCKER_MYSQL_DATABASE)
                    .and_then(|value: &serde_yaml::Value| value.as_str())
                    .map(String::from)
                {
                    config.set_mysql_database(Some(database));
                }
                if let Some(username) = env
                    .get(DOCKER_MYSQL_USER)
                    .and_then(|value: &serde_yaml::Value| value.as_str())
                    .map(String::from)
                {
                    config.set_mysql_username(Some(username));
                }
                if let Some(password) = env
                    .get(DOCKER_MYSQL_PASSWORD)
                    .and_then(|value: &serde_yaml::Value| value.as_str())
                    .map(String::from)
                {
                    config.set_mysql_password(Some(password));
                }
            }
            if let Some(ports) = mysql
                .get(DOCKER_YAML_PORTS)
                .and_then(|ports_value: &serde_yaml::Value| ports_value.as_sequence())
                && let Some(port_mapping) = ports
                    .first()
                    .and_then(|port: &serde_yaml::Value| port.as_str())
                && let Some(host_port) = port_mapping.split(COLON).next()
                && let Ok(port) = host_port.parse()
            {
                config.set_mysql_port(Some(port));
            }
        }
        if let Some(postgresql) = yaml
            .get(DOCKER_YAML_SERVICES)
            .and_then(|services: &serde_yaml::Value| services.get(DOCKER_SERVICE_POSTGRESQL))
        {
            if let Some(env) = postgresql.get(DOCKER_YAML_ENVIRONMENT) {
                if let Some(database) = env
                    .get(DOCKER_POSTGRES_DB)
                    .and_then(|value: &serde_yaml::Value| value.as_str())
                    .map(String::from)
                {
                    config.set_postgresql_database(Some(database));
                }
                if let Some(username) = env
                    .get(DOCKER_POSTGRES_USER)
                    .and_then(|value: &serde_yaml::Value| value.as_str())
                    .map(String::from)
                {
                    config.set_postgresql_username(Some(username));
                }
                if let Some(password) = env
                    .get(DOCKER_POSTGRES_PASSWORD)
                    .and_then(|value: &serde_yaml::Value| value.as_str())
                    .map(String::from)
                {
                    config.set_postgresql_password(Some(password));
                }
            }
            if let Some(ports) = postgresql
                .get(DOCKER_YAML_PORTS)
                .and_then(|ports_value: &serde_yaml::Value| ports_value.as_sequence())
                && let Some(port_mapping) = ports
                    .first()
                    .and_then(|port: &serde_yaml::Value| port.as_str())
                && let Some(host_port) = port_mapping.split(COLON).next()
                && let Ok(port) = host_port.parse()
            {
                config.set_postgresql_port(Some(port));
            }
        }
        if let Some(redis) = yaml
            .get(DOCKER_YAML_SERVICES)
            .and_then(|services: &serde_yaml::Value| services.get(DOCKER_SERVICE_REDIS))
        {
            if let Some(command) = redis
                .get(DOCKER_YAML_COMMAND)
                .and_then(|command_value: &serde_yaml::Value| command_value.as_str())
                && let Some(password_part) = command.split(DOCKER_REDIS_PASSWORD_FLAG).nth(1)
            {
                config.set_redis_password(Some(password_part.trim().to_string()));
            }
            if let Some(ports) = redis
                .get(DOCKER_YAML_PORTS)
                .and_then(|ports_value: &serde_yaml::Value| ports_value.as_sequence())
                && let Some(port_mapping) = ports
                    .first()
                    .and_then(|port: &serde_yaml::Value| port.as_str())
                && let Some(host_port) = port_mapping.split(COLON).next()
                && let Ok(port) = host_port.parse()
            {
                config.set_redis_port(Some(port));
            }
        }
        Ok(config)
    }

    /// Logs the current environment configuration, with full details in debug mode and masked passwords in release mode.
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
                "  GPT_API_KEY: {}",
                if config.get_gpt_api_key().is_empty() {
                    "(not set)"
                } else {
                    "***"
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
            info!(
                "  GPT_ENABLE_THINKING: {}",
                config.get_gpt_enable_thinking()
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
                "GPT API Key {}",
                if config.get_gpt_api_key().is_empty() {
                    "(not set)"
                } else {
                    "***"
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
            info!("GPT Enable Thinking {}", config.get_gpt_enable_thinking());
            info!("MySQL Configuration:");
            if config.get_mysql_instances().is_empty() {
                info!("  (no MySQL instances configured)");
            } else {
                for instance in config.get_mysql_instances() {
                    info!(
                        "  Instance '{}' {}:***@{}:{}/{}",
                        instance.get_name(),
                        instance.get_username(),
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
                        "  Instance '{}' {}:***@{}:{}/{}",
                        instance.get_name(),
                        instance.get_username(),
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
                        "  Instance '{}' {}:***@{}:{}",
                        instance.get_name(),
                        if instance.get_username().is_empty() {
                            "(none)"
                        } else {
                            instance.get_username()
                        },
                        instance.get_host(),
                        instance.get_port()
                    );
                }
            }
        }
    }
}
