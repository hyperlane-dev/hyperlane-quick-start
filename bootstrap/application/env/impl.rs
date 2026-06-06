use super::*;

/// Implementation of `BootstrapSyncInit` for `EnvBootstrap`, loading the environment configuration on initialization.
impl BootstrapSyncInit for EnvBootstrap {
    fn init() -> Self {
        if let Err(error) = EnvPlugin::try_load_config() {
            panic!("{error}");
        }
        Self
    }
}
