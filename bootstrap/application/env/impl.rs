use super::*;

impl BootstrapSyncInit for EnvBootstrap {
    fn init() -> Self {
        if let Err(error) = EnvPlugin::try_load_config() {
            panic!("{error}");
        }
        Self
    }
}
