use super::*;

#[instrument_trace]
pub fn init_env_config() -> Result<(), String> {
    EnvPlugin::try_get_config()
}
