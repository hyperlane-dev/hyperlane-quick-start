use super::*;

pub fn get_global_env_config() -> &'static EnvConfig {
    GLOBAL_ENV_CONFIG.get_or_init(EnvConfig::default)
}

pub fn init_env_config() -> Result<(), String> {
    let config: EnvConfig = EnvConfig::load()?;
    GLOBAL_ENV_CONFIG
        .set(config.clone())
        .map_err(|_| "Failed to initialize global environment configuration".to_string())?;
    info!("Environment Configuration Loaded Successfully");
    info!(
        "GPT API URL: {}",
        if config.get_gpt_api_url().is_empty() {
            "(not set)"
        } else {
            config.get_gpt_api_url()
        }
    );
    info!(
        "GPT Model: {}",
        if config.get_gpt_model().is_empty() {
            "(not set)"
        } else {
            config.get_gpt_model()
        }
    );
    info!("MySQL Configuration:");
    info!(
        "  Host: {}",
        if config.get_mysql_host().is_empty() {
            "(not set)"
        } else {
            config.get_mysql_host()
        }
    );
    info!("  Port: {}", config.get_mysql_port());
    info!(
        "  Database: {}",
        if config.get_mysql_database().is_empty() {
            "(not set)"
        } else {
            config.get_mysql_database()
        }
    );
    info!(
        "  Username: {}",
        if config.get_mysql_username().is_empty() {
            "(not set)"
        } else {
            config.get_mysql_username()
        }
    );
    info!(
        "  Password: {}",
        if config.get_mysql_password().is_empty() {
            "(not set)"
        } else {
            "***"
        }
    );

    info!("PostgreSQL Configuration:");
    info!(
        "  Host: {}",
        if config.get_postgresql_host().is_empty() {
            "(not set)"
        } else {
            config.get_postgresql_host()
        }
    );
    info!("  Port: {}", config.get_postgresql_port());
    info!(
        "  Database: {}",
        if config.get_postgresql_database().is_empty() {
            "(not set)"
        } else {
            config.get_postgresql_database()
        }
    );
    info!(
        "  Username: {}",
        if config.get_postgresql_username().is_empty() {
            "(not set)"
        } else {
            config.get_postgresql_username()
        }
    );
    info!(
        "  Password: {}",
        if config.get_postgresql_password().is_empty() {
            "(not set)"
        } else {
            "***"
        }
    );
    info!("Redis Configuration:");
    info!(
        "  Host: {}",
        if config.get_redis_host().is_empty() {
            "(not set)"
        } else {
            config.get_redis_host()
        }
    );
    info!("  Port: {}", config.get_redis_port());
    info!(
        "  Username: {}",
        if config.get_redis_username().is_empty() {
            "(not set)"
        } else {
            config.get_redis_username()
        }
    );
    info!(
        "  Password: {}",
        if config.get_redis_password().is_empty() {
            "(not set)"
        } else {
            "***"
        }
    );
    Ok(())
}
