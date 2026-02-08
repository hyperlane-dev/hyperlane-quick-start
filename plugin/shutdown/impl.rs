use super::*;

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
    pub fn get_or_init() -> SharedAsyncTaskFactory<()> {
        SHUTDOWN.get_or_init(Self::get_init).clone()
    }

    #[instrument_trace]
    pub fn set(shutdown: &SharedAsyncTaskFactory<()>) {
        drop(SHUTDOWN.set(shutdown.clone()));
    }
}
