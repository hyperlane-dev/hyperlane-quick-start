use super::*;

#[instrument_trace]
pub async fn initialize_auto_creation() -> Result<(), String> {
    if let Err(error) = AutoCreationConfig::validate() {
        return Err(format!(
            "Auto-creation configuration validation failed{COLON_SPACE}{error}"
        ));
    }
    let env: &'static EnvConfig = get_global_env_config();
    let mut initialization_results: Vec<String> = Vec::new();
    for instance in &env.mysql_instances {
        match mysql::perform_mysql_auto_creation(instance).await {
            Ok(result) => {
                initialization_results.push(format!(
                    "MySQL ({}) {COLON_SPACE}{}",
                    instance.name,
                    if result.has_changes() {
                        "initialized with changes"
                    } else {
                        "verified"
                    }
                ));
            }
            Err(error) => {
                if !error.should_continue() {
                    return Err(format!(
                        "MySQL ({}) auto-creation failed{COLON_SPACE}{error}",
                        instance.name
                    ));
                }
                initialization_results.push(format!(
                    "MySQL ({}) : failed but continuing ({error})",
                    instance.name
                ));
            }
        }
    }
    for instance in &env.postgresql_instances {
        match postgresql::perform_postgresql_auto_creation(instance).await {
            Ok(result) => {
                initialization_results.push(format!(
                    "PostgreSQL ({}) {COLON_SPACE}{}",
                    instance.name,
                    if result.has_changes() {
                        "initialized with changes"
                    } else {
                        "verified"
                    }
                ));
            }
            Err(error) => {
                if !error.should_continue() {
                    return Err(format!(
                        "PostgreSQL ({}) auto-creation failed{COLON_SPACE}{error}",
                        instance.name
                    ));
                }
                initialization_results.push(format!(
                    "PostgreSQL ({}) : failed but continuing ({error})",
                    instance.name
                ));
            }
        }
    }
    for instance in &env.redis_instances {
        match redis::perform_redis_auto_creation(instance).await {
            Ok(result) => {
                initialization_results.push(format!(
                    "Redis ({}) {COLON_SPACE}{}",
                    instance.name,
                    if result.has_changes() {
                        "initialized with changes"
                    } else {
                        "verified"
                    }
                ));
            }
            Err(error) => {
                if !error.should_continue() {
                    return Err(format!(
                        "Redis ({}) auto-creation failed{COLON_SPACE}{error}",
                        instance.name
                    ));
                }
                initialization_results.push(format!(
                    "Redis ({}) : failed but continuing ({error})",
                    instance.name
                ));
            }
        }
    }
    if initialization_results.is_empty() {
        info!("[AUTO-CREATION] No plugins enabled for auto-creation");
    } else {
        let results_summary: String = initialization_results.join(", ");
        info!("[AUTO-CREATION] Initialization complete{COLON_SPACE}{results_summary}");
    }
    Ok(())
}
