use super::*;

impl EnvBootstrap {
    #[instrument_trace]
    pub fn init() -> Result<(), String> {
        EnvPlugin::try_get_config()
    }
}
