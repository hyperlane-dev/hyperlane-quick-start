use super::*;

impl EnvConfig {
    pub fn load() -> Result<Self, String> {
        let env_content: Vec<u8> = match read_from_file(ENV_FILE_PATH) {
            Ok(content) => content,
            Err(_) => {
                let example_content: &str = "GPT_API_URL=";
                let _ = write_to_file(ENV_FILE_PATH, example_content.as_bytes())
                    .map_err(|e| format!("Failed to create example env file: {}", e))?;
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
            gpt_api_url: config_map.get("GPT_API_URL").cloned().unwrap_or_default(),
            gtp_model: config_map.get("GPT_MODEL").cloned().unwrap_or_default(),
            mysql_host: config_map.get("MYSQL_HOST").cloned().unwrap_or_default(),
            mysql_port: config_map
                .get("MYSQL_PORT")
                .cloned()
                .unwrap_or_default()
                .parse::<u16>()
                .map_err(|_| "MYSQL_PORT must be a valid 16-bit unsigned integer")?,
            mysql_database: config_map
                .get("MYSQL_DATABASE")
                .cloned()
                .unwrap_or_default(),
            mysql_username: config_map
                .get("MYSQL_USERNAME")
                .cloned()
                .unwrap_or_default(),
            mysql_password: config_map
                .get("MYSQL_PASSWORD")
                .cloned()
                .unwrap_or_default(),
            redis_host: config_map.get("REDIS_HOST").cloned().unwrap_or_default(),
            redis_port: config_map
                .get("REDIS_PORT")
                .cloned()
                .unwrap_or_default()
                .parse::<u16>()
                .map_err(|_| "REDIS_PORT must be a valid 16-bit unsigned integer")?,
            redis_username: config_map
                .get("REDIS_USERNAME")
                .cloned()
                .unwrap_or_default(),
            redis_password: config_map
                .get("REDIS_PASSWORD")
                .cloned()
                .unwrap_or_default(),
            postgresql_host: config_map.get("POSTGRES_HOST").cloned().unwrap_or_default(),
            postgresql_port: config_map
                .get("POSTGRES_PORT")
                .cloned()
                .unwrap_or_default()
                .parse::<u16>()
                .map_err(|_| "POSTGRES_PORT must be a valid 16-bit unsigned integer")?,
            postgresql_database: config_map
                .get("POSTGRES_DATABASE")
                .cloned()
                .unwrap_or_default(),
            postgresql_username: config_map
                .get("POSTGRES_USERNAME")
                .cloned()
                .unwrap_or_default(),
            postgresql_password: config_map
                .get("POSTGRES_PASSWORD")
                .cloned()
                .unwrap_or_default(),
        })
    }
}
