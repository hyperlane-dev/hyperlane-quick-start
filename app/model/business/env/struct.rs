use super::*;

#[derive(Debug, Clone)]
pub struct EnvConfig {
    pub gpt_api_url: String,
    pub gpt_api_key: String,
}

impl EnvConfig {
    pub fn load() -> Result<Self, String> {
        let env_content = match fs::read_to_string(WS_ENV_FILE_PATH) {
            Ok(content) => content,
            Err(_) => {
                let example_content = "GPT_API_URL=https://api.cloudflare.com/client/v4/accounts/YOUR_ACCOUNT_ID/ai/run/@cf/meta/llama-4-scout-17b-16e-instruct\nGPT_API_KEY=YOUR_API_KEY_HERE";
                let _ = write_to_file(env_path, example_content.as_bytes())
                    .map_err(|e| format!("Failed to create example env file: {}", e))?;
                return Err(format!(
                    "Environment file {} was not found. Created example file. Please update it with your actual API credentials.",
                    env_path
                ));
            }
        };
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
