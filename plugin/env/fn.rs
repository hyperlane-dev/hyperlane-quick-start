use super::*;

#[instrument_trace]
pub fn get_or_init_global_env_config() -> &'static EnvConfig {
    GLOBAL_ENV_CONFIG.get_or_init(EnvConfig::default)
}

#[instrument_trace]
pub fn load_env_config() -> Result<(), String> {
    let config: EnvConfig = EnvConfig::load()?;
    GLOBAL_ENV_CONFIG
        .set(config.clone())
        .map_err(|_| "Failed to initialize global environment configuration".to_string())?;
    info!("Environment Configuration Loaded Successfully");
    info!(
        "GPT API URL {}",
        if config.get_gpt_api_url().is_empty() {
            "(not set)"
        } else {
            config.get_gpt_api_url()
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
    info!("MySQL Configuration:");
    if config.get_mysql_instances().is_empty() {
        info!("  (no MySQL instances configured)");
    } else {
        for instance in config.get_mysql_instances() {
            info!(
                "  Instance '{}' {}:{}@{}:{}/{}",
                instance.get_name(),
                instance.get_username(),
                "***",
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
                "  Instance '{}' {}:{}@{}:{}/{}",
                instance.get_name(),
                instance.get_username(),
                "***",
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
                "  Instance '{}' {}:{}@{}:{}",
                instance.get_name(),
                if instance.get_username().is_empty() {
                    "(none)"
                } else {
                    instance.get_username()
                },
                "***",
                instance.get_host(),
                instance.get_port()
            );
        }
    }
    Ok(())
}
