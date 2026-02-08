use super::*;

impl BootstrapSyncInit for EnvBootstrap {
    fn init() -> Self {
        if let Err(error) = EnvPlugin::try_get_config() {
            error!("{error}");
        }
        Self
    }
}
