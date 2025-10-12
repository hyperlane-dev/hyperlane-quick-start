use super::*;

pub async fn initialize_auto_creation() -> Result<(), String> {
    if let Err(error) = AutoCreationConfig::validate() {
        return Err(format!(
            "Auto-creation configuration validation failed: {}",
            error
        ));
    }
    let summary: String = AutoCreationConfig::get_summary();
    log_info(&format!("[AUTO-CREATION] Initialization: {}", summary)).await;
    if !AutoCreationConfig::is_auto_creation_enabled() {
        log_info("[AUTO-CREATION] Auto-creation is disabled for all plugins").await;
        return Ok(());
    }
    let env: &'static EnvConfig = get_global_env_config();
    let mut initialization_results: Vec<String> = Vec::new();
    if env.enable_mysql && AutoCreationConfig::is_auto_creation_enabled() {
        match perform_mysql_auto_creation().await {
            Ok(result) => {
                initialization_results.push(format!(
                    "MySQL: {}",
                    if result.has_changes() {
                        "initialized with changes"
                    } else {
                        "verified"
                    }
                ));
            }
            Err(error) => {
                if !error.should_continue() {
                    return Err(format!("MySQL auto-creation failed: {}", error));
                }
                initialization_results.push(format!("MySQL: failed but continuing ({})", error));
            }
        }
    }
    if env.enable_postgresql && AutoCreationConfig::is_auto_creation_enabled() {
        match perform_postgresql_auto_creation().await {
            Ok(result) => {
                initialization_results.push(format!(
                    "PostgreSQL: {}",
                    if result.has_changes() {
                        "initialized with changes"
                    } else {
                        "verified"
                    }
                ));
            }
            Err(error) => {
                if !error.should_continue() {
                    return Err(format!("PostgreSQL auto-creation failed: {}", error));
                }
                initialization_results
                    .push(format!("PostgreSQL: failed but continuing ({})", error));
            }
        }
    }
    if env.enable_redis && AutoCreationConfig::is_auto_creation_enabled() {
        match perform_redis_auto_creation().await {
            Ok(result) => {
                initialization_results.push(format!(
                    "Redis: {}",
                    if result.has_changes() {
                        "initialized with changes"
                    } else {
                        "verified"
                    }
                ));
            }
            Err(error) => {
                if !error.should_continue() {
                    return Err(format!("Redis auto-creation failed: {}", error));
                }
                initialization_results.push(format!("Redis: failed but continuing ({})", error));
            }
        }
    }
    if initialization_results.is_empty() {
        log_info("[AUTO-CREATION] No plugins enabled for auto-creation").await;
    } else {
        let results_summary: String = initialization_results.join(", ");
        log_info(&format!(
            "[AUTO-CREATION] Initialization complete: {}",
            results_summary
        ))
        .await;
    }
    Ok(())
}
