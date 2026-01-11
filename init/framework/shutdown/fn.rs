use super::*;

fn default_shutdown() -> SharedAsyncTaskFactory<()> {
    Arc::new(|| {
        Box::pin(async {
            warn!("Not set shutdown, using default");
        })
    })
}

pub fn set_shutdown(shutdown: SharedAsyncTaskFactory<()>) {
    trace!("Set shutdown");
    drop(SHUTDOWN.set(shutdown));
}

pub fn get_shutdown() -> SharedAsyncTaskFactory<()> {
    trace!("Get shutdown");
    SHUTDOWN.get_or_init(default_shutdown).clone()
}
