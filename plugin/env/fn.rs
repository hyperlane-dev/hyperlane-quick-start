use super::*;

pub fn get_global_env_config() -> &'static EnvConfig {
    GLOBAL_ENV_CONFIG.get_or_init(EnvConfig::default)
}

pub fn init_env_config() -> Result<(), String> {
    let config: EnvConfig = EnvConfig::load()?;
    GLOBAL_ENV_CONFIG
        .set(config.clone())
        .map_err(|_| "Failed to initialize global environment configuration".to_string())?;
    println_success!("Environment Configuration Loaded Successfully");
    println_success!(
        "GPT API URL: {}",
        if config.get_gpt_api_url().is_empty() {
            "(not set)"
        } else {
            config.get_gpt_api_url()
        }
    );
    println_success!(
        "GPT Model: {}",
        if config.get_gtp_model().is_empty() {
            "(not set)"
        } else {
            config.get_gtp_model()
        }
    );
    println_success!("MySQL Enabled: {}", config.get_enable_mysql());
    if *config.get_enable_mysql() {
        println_success!(
            "  Host: {}",
            if config.get_mysql_host().is_empty() {
                "(not set)"
            } else {
                config.get_mysql_host()
            }
        );
        println_success!("  Port: {}", config.get_mysql_port());
        println_success!(
            "  Database: {}",
            if config.get_mysql_database().is_empty() {
                "(not set)"
            } else {
                config.get_mysql_database()
            }
        );
        println_success!(
            "  Username: {}",
            if config.get_mysql_username().is_empty() {
                "(not set)"
            } else {
                config.get_mysql_username()
            }
        );
        println_success!(
            "  Password: {}",
            if config.get_mysql_password().is_empty() {
                "(not set)"
            } else {
                "***"
            }
        );
    }
    println_success!("PostgreSQL Enabled: {}", config.get_enable_postgresql());
    if *config.get_enable_postgresql() {
        println_success!(
            "  Host: {}",
            if config.get_postgresql_host().is_empty() {
                "(not set)"
            } else {
                config.get_postgresql_host()
            }
        );
        println_success!("  Port: {}", config.get_postgresql_port());
        println_success!(
            "  Database: {}",
            if config.get_postgresql_database().is_empty() {
                "(not set)"
            } else {
                config.get_postgresql_database()
            }
        );
        println_success!(
            "  Username: {}",
            if config.get_postgresql_username().is_empty() {
                "(not set)"
            } else {
                config.get_postgresql_username()
            }
        );
        println_success!(
            "  Password: {}",
            if config.get_postgresql_password().is_empty() {
                "(not set)"
            } else {
                "***"
            }
        );
    }
    println_success!("Redis Enabled: {}", config.get_enable_redis());
    if *config.get_enable_redis() {
        println_success!(
            "  Host: {}",
            if config.get_redis_host().is_empty() {
                "(not set)"
            } else {
                config.get_redis_host()
            }
        );
        println_success!("  Port: {}", config.get_redis_port());
        println_success!(
            "  Username: {}",
            if config.get_redis_username().is_empty() {
                "(not set)"
            } else {
                config.get_redis_username()
            }
        );
        println_success!(
            "  Password: {}",
            if config.get_redis_password().is_empty() {
                "(not set)"
            } else {
                "***"
            }
        );
    }
    Ok(())
}
