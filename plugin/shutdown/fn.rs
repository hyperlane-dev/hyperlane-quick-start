use super::*;

#[instrument_trace]
fn default_shutdown() -> SharedAsyncTaskFactory<()> {
    Arc::new(|| {
        Box::pin(async {
            warn!("Not set shutdown, using default");
        })
    })
}

#[instrument_trace]
pub fn set_shutdown(shutdown: &SharedAsyncTaskFactory<()>) {
    drop(SHUTDOWN.set(shutdown.clone()));
}

#[instrument_trace]
pub fn get_shutdown() -> SharedAsyncTaskFactory<()> {
    SHUTDOWN.get_or_init(default_shutdown).clone()
}
