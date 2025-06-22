use super::*;

pub static GLOBAL_ENV_CONFIG: OnceLock<EnvConfig> = OnceLock::new();

pub fn get_global_env_config() -> &'static EnvConfig {
    GLOBAL_ENV_CONFIG.get_or_init(|| {
        EnvConfig::load().unwrap_or_else(|e| {
            panic!("Failed to load environment configuration: {}", e);
        })
    })
}

pub fn init_env_config() -> Result<(), String> {
    let config = EnvConfig::load()?;
    GLOBAL_ENV_CONFIG.set(config).map_err(|_| {
        "Failed to initialize global environment configuration".to_string()
    })?;
    Ok(())
}
