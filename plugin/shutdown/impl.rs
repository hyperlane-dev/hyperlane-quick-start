use super::*;

impl GetOrInit for ShutdownPlugin {
    type Instance = SharedAsyncTaskFactory<()>;

    fn get_or_init() -> &'static Self::Instance {
        SHUTDOWN.get_or_init(Self::get_init)
    }
}

impl ShutdownPlugin {
    #[instrument_trace]
    pub fn get_init() -> SharedAsyncTaskFactory<()> {
        Arc::new(|| {
            Box::pin(async {
                warn!("Not set shutdown, using default");
            })
        })
    }

    #[instrument_trace]
    pub fn set(shutdown: &SharedAsyncTaskFactory<()>) {
        drop(SHUTDOWN.set(shutdown.clone()));
    }
}
