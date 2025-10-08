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
        let env_content: Cow<'_, str> = String::from_utf8_lossy(&env_content);
        let mut config_map: HashMap<String, String> = HashMap::new();
        for line in env_content.lines() {
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
                .get("GPT_API_URL")
                .cloned()
                .ok_or("GPT_API_URL not found in /shell/env")?,
            gtp_model: config_map
                .get("GPT_MODEL")
                .cloned()
                .ok_or("GPT_MODEL not found in /shell/env")?,
            mysql_host: config_map
                .get("MYSQL_HOST")
                .cloned()
                .ok_or("MYSQL_HOST not found in /shell/env")?,
            mysql_port: config_map
                .get("MYSQL_PORT")
                .cloned()
                .ok_or("MYSQL_PORT not found in /shell/env")?
                .parse::<u16>()
                .map_err(|_| "MYSQL_PORT must be a valid 16-bit unsigned integer")?,
            mysql_database: config_map
                .get("MYSQL_DATABASE")
                .cloned()
                .ok_or("MYSQL_DATABASE not found in /shell/env")?,
            mysql_username: config_map
                .get("MYSQL_USERNAME")
                .cloned()
                .ok_or("MYSQL_USERNAME not found in /shell/env")?,
            mysql_password: config_map
                .get("MYSQL_PASSWORD")
                .cloned()
                .ok_or("MYSQL_PASSWORD not found in /shell/env")?,
            redis_host: config_map
                .get("REDIS_HOST")
                .cloned()
                .ok_or("REDIS_HOST not found in /shell/env")?,
            redis_port: config_map
                .get("REDIS_PORT")
                .cloned()
                .ok_or("REDIS_PORT not found in /shell/env")?
                .parse::<u16>()
                .map_err(|_| "REDIS_PORT must be a valid 16-bit unsigned integer")?,
            redis_username: config_map
                .get("REDIS_USERNAME")
                .cloned()
                .ok_or("REDIS_USERNAME not found in /shell/env")?,
            redis_password: config_map
                .get("REDIS_PASSWORD")
                .cloned()
                .ok_or("REDIS_PASSWORD not found in /shell/env")?,
            postgresql_host: config_map
                .get("POSTGRES_HOST")
                .cloned()
                .ok_or("POSTGRES_HOST not found in /shell/env")?,
            postgresql_port: config_map
                .get("POSTGRES_PORT")
                .cloned()
                .ok_or("POSTGRES_PORT not found in /shell/env")?
                .parse::<u16>()
                .map_err(|_| "POSTGRES_PORT must be a valid 16-bit unsigned integer")?,
            postgresql_database: config_map
                .get("POSTGRES_DATABASE")
                .cloned()
                .ok_or("POSTGRES_DATABASE not found in /shell/env")?,
            postgresql_username: config_map
                .get("POSTGRES_USERNAME")
                .cloned()
                .ok_or("POSTGRES_USERNAME not found in /shell/env")?,
            postgresql_password: config_map
                .get("POSTGRES_PASSWORD")
                .cloned()
                .ok_or("POSTGRES_PASSWORD not found in /shell/env")?,
        })
    }
}
