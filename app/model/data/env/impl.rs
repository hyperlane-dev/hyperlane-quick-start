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
        let gpt_api_url: String = config_map
            .get("GPT_API_URL")
            .ok_or("GPT_API_URL not found in /shell/env")?
            .clone();
        let gtp_model: String = config_map
            .get("GPT_MODEL")
            .ok_or("GPT_MODEL not found in /shell/env")?
            .clone();
        Ok(EnvConfig {
            gpt_api_url,
            gtp_model,
        })
    }
}
