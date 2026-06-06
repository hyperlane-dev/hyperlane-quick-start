use super::*;

/// Implementation of `BootstrapSyncInit` for `EnvBootstrap`, loading the environment configuration on initialization.
impl BootstrapSyncInit for EnvBootstrap {
    /// Initializes the environment bootstrap by loading the environment configuration from the env file.
    ///
    /// # Panics
    ///
    /// Panics if the environment configuration fails to load.
    ///
    /// # Returns
    ///
    /// - `Self`: The initialized `EnvBootstrap` instance.
    fn init() -> Self {
        if let Err(error) = EnvPlugin::try_load_config() {
            panic!("{error}");
        }
        Self
    }
}
