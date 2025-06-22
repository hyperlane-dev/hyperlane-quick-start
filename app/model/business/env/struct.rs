use super::*;

#[derive(Debug, Clone)]
pub struct EnvConfig {
    pub gpt_api_url: String,
    pub gpt_api_key: String,
}

impl EnvConfig {
    pub fn load() -> Result<Self, String> {
        let env_content = fs::read_to_string("/shell/env")
            .map_err(|e| format!("Failed to read /shell/env file: {}", e))?;
        
        let mut config_map = HashMap::new();
        for line in env_content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((key, value)) = line.split_once('=') {
                config_map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        
        let gpt_api_url = config_map
            .get("GPT_API_URL")
            .ok_or("GPT_API_URL not found in /shell/env")?
            .clone();
        
        let gpt_api_key = config_map
            .get("GPT_API_KEY")
            .ok_or("GPT_API_KEY not found in /shell/env")?
            .clone();
        
        Ok(EnvConfig {
            gpt_api_url,
            gpt_api_key,
        })
    }
}
