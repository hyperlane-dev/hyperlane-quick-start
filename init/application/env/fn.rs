use super::*;

#[instrument_trace]
pub fn init_env_config() -> Result<(), String> {
    load_env_config()
}
