use super::*;

#[instrument_trace]
pub fn get_global_env_config() -> &'static EnvConfig {
    GLOBAL_ENV_CONFIG.get_or_init(EnvConfig::default)
}

#[instrument_trace]
pub fn init_env_config() -> Result<(), String> {
    let config: EnvConfig = EnvConfig::load()?;
    GLOBAL_ENV_CONFIG
        .set(config.clone())
        .map_err(|_| "Failed to initialize global environment configuration".to_string())?;
    info!("Environment Configuration Loaded Successfully");
    info!(
        "GPT API URL: {}",
        if config.gpt_api_url.is_empty() {
            "(not set)"
        } else {
            &config.gpt_api_url
        }
    );
    info!(
        "GPT Model: {}",
        if config.gpt_model.is_empty() {
            "(not set)"
        } else {
            &config.gpt_model
        }
    );

    info!("MySQL Configuration:");
    if config.mysql_instances.is_empty() {
        info!("  (no MySQL instances configured)");
    } else {
        for instance in &config.mysql_instances {
            info!(
                "  Instance '{}': {}:{}@{}:{}/{}",
                instance.name,
                instance.username,
                "***",
                instance.host,
                instance.port,
                instance.database
            );
        }
    }

    info!("PostgreSQL Configuration:");
    if config.postgresql_instances.is_empty() {
        info!("  (no PostgreSQL instances configured)");
    } else {
        for instance in &config.postgresql_instances {
            info!(
                "  Instance '{}': {}:{}@{}:{}/{}",
                instance.name,
                instance.username,
                "***",
                instance.host,
                instance.port,
                instance.database
            );
        }
    }

    info!("Redis Configuration:");
    if config.redis_instances.is_empty() {
        info!("  (no Redis instances configured)");
    } else {
        for instance in &config.redis_instances {
            info!(
                "  Instance '{}': {}:{}@{}:{}",
                instance.name,
                if instance.username.is_empty() {
                    "(none)"
                } else {
                    &instance.username
                },
                "***",
                instance.host,
                instance.port
            );
        }
    }
    Ok(())
}
