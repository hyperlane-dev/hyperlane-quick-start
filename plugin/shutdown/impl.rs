use super::*;

/// Implementation of `GetOrInit` for `ShutdownPlugin`, providing lazy initialization of the shutdown hook handler.
impl GetOrInit for ShutdownPlugin {
    type Instance = ServerControlHookHandler<()>;

    fn get_or_init() -> &'static Self::Instance {
        SHUTDOWN.get_or_init(Self::get_init)
    }
}

/// Implementation of shutdown hook management methods for `ShutdownPlugin`.
impl ShutdownPlugin {
    /// Returns the default shutdown hook handler that logs a warning when no custom shutdown is configured.
    ///
    /// # Returns
    ///
    /// - `ServerControlHookHandler<()>`: The default shutdown hook handler.
    #[instrument_trace]
    pub fn get_init() -> ServerControlHookHandler<()> {
        Arc::new(|| {
            Box::pin(async {
                warn!("Not set shutdown, using default");
            })
        })
    }

    /// Sets the global shutdown hook handler to the provided handler.
    ///
    /// # Arguments
    ///
    /// - `&ServerControlHookHandler<()>`: The shutdown hook handler to set.
    #[instrument_trace]
    pub fn set(shutdown: &ServerControlHookHandler<()>) {
        drop(SHUTDOWN.set(shutdown.clone()));
    }
}
