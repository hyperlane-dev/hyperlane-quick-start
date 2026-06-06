use super::*;

/// Global static storage for the `EnvConfig` singleton, initialized once.
pub static GLOBAL_ENV_CONFIG: OnceLock<EnvConfig> = OnceLock::new();
