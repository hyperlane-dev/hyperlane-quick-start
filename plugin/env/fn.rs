use super::*;

pub fn get_global_env_config() -> &'static EnvConfig {
    GLOBAL_ENV_CONFIG.get_or_init(EnvConfig::default)
}

pub fn init_env_config() -> Result<(), String> {
    let config: EnvConfig = EnvConfig::load()?;
    GLOBAL_ENV_CONFIG
        .set(config)
        .map_err(|_| "Failed to initialize global environment configuration".to_string())?;
    Ok(())
}
