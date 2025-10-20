use super::*;

impl EnvConfig {
    pub fn load() -> Result<Self, String> {
        let env_content: Vec<u8> = match read_from_file(ENV_FILE_PATH) {
            Ok(content) => content,
            Err(_) => {
                let data: String = format!(
                    "{ENV_KEY_GPT_API_URL}=\n\
                     {ENV_KEY_GPT_MODEL}=\n\
                     {ENV_KEY_MYSQL_HOST}=\n\
                     {ENV_KEY_MYSQL_PORT}=\n\
                     {ENV_KEY_MYSQL_DATABASE}=\n\
                     {ENV_KEY_MYSQL_USERNAME}=\n\
                     {ENV_KEY_MYSQL_PASSWORD}=\n\
                     {ENV_KEY_REDIS_HOST}=\n\
                     {ENV_KEY_REDIS_PORT}=\n\
                     {ENV_KEY_REDIS_USERNAME}=\n\
                     {ENV_KEY_REDIS_PASSWORD}=\n\
                     {ENV_KEY_POSTGRES_HOST}=\n\
                     {ENV_KEY_POSTGRES_PORT}=\n\
                     {ENV_KEY_POSTGRES_DATABASE}=\n\
                     {ENV_KEY_POSTGRES_USERNAME}=\n\
                     {ENV_KEY_POSTGRES_PASSWORD}=\n\
                     {ENV_KEY_ENABLE_MYSQL}=\n\
                     {ENV_KEY_ENABLE_REDIS}=\n\
                     {ENV_KEY_ENABLE_POSTGRESQL}=\n",
                );
                write_to_file(ENV_FILE_PATH, data.as_bytes())
                    .map_err(|error| format!("Failed to create example env file: {error}"))?;
                return Self::load();
            }
        };
        let mut config_map: HashMap<String, String> = HashMap::new();
        for line in String::from_utf8_lossy(&env_content).lines() {
            let line: &str = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                config_map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        Ok(EnvConfig {
            gpt_api_url: config_map
                .get(ENV_KEY_GPT_API_URL)
                .cloned()
                .unwrap_or_default(),
            enable_mysql: config_map
                .get(ENV_KEY_ENABLE_MYSQL)
                .cloned()
                .unwrap_or_default()
                .parse::<bool>()
                .unwrap_or(false),
            enable_redis: config_map
                .get(ENV_KEY_ENABLE_REDIS)
                .cloned()
                .unwrap_or_default()
                .parse::<bool>()
                .unwrap_or(false),
            enable_postgresql: config_map
                .get(ENV_KEY_ENABLE_POSTGRESQL)
                .cloned()
                .unwrap_or_default()
                .parse::<bool>()
                .unwrap_or_default(),
            gtp_model: config_map
                .get(ENV_KEY_GPT_MODEL)
                .cloned()
                .unwrap_or_default(),
            mysql_host: config_map
                .get(ENV_KEY_MYSQL_HOST)
                .cloned()
                .unwrap_or_default(),
            mysql_port: config_map
                .get(ENV_KEY_MYSQL_PORT)
                .cloned()
                .unwrap_or_default()
                .parse::<usize>()
                .map_err(|_| "MYSQL_PORT must be a valid 16-bit unsigned integer")?,
            mysql_database: config_map
                .get(ENV_KEY_MYSQL_DATABASE)
                .cloned()
                .unwrap_or_default(),
            mysql_username: config_map
                .get(ENV_KEY_MYSQL_USERNAME)
                .cloned()
                .unwrap_or_default(),
            mysql_password: config_map
                .get(ENV_KEY_MYSQL_PASSWORD)
                .cloned()
                .unwrap_or_default(),
            redis_host: config_map
                .get(ENV_KEY_REDIS_HOST)
                .cloned()
                .unwrap_or_default(),
            redis_port: config_map
                .get(ENV_KEY_REDIS_PORT)
                .cloned()
                .unwrap_or_default()
                .parse::<usize>()
                .map_err(|_| "REDIS_PORT must be a valid 16-bit unsigned integer")?,
            redis_username: config_map
                .get(ENV_KEY_REDIS_USERNAME)
                .cloned()
                .unwrap_or_default(),
            redis_password: config_map
                .get(ENV_KEY_REDIS_PASSWORD)
                .cloned()
                .unwrap_or_default(),
            postgresql_host: config_map
                .get(ENV_KEY_POSTGRES_HOST)
                .cloned()
                .unwrap_or_default(),
            postgresql_port: config_map
                .get(ENV_KEY_POSTGRES_PORT)
                .cloned()
                .unwrap_or_default()
                .parse::<usize>()
                .map_err(|_| "POSTGRES_PORT must be a valid 16-bit unsigned integer")?,
            postgresql_database: config_map
                .get(ENV_KEY_POSTGRES_DATABASE)
                .cloned()
                .unwrap_or_default(),
            postgresql_username: config_map
                .get(ENV_KEY_POSTGRES_USERNAME)
                .cloned()
                .unwrap_or_default(),
            postgresql_password: config_map
                .get(ENV_KEY_POSTGRES_PASSWORD)
                .cloned()
                .unwrap_or_default(),
        })
    }
}
