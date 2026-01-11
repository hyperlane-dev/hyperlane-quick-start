use super::*;

pub fn set_shutdown(shutdown: SharedAsyncTaskFactory<()>) {
    trace!("Set shutdown");
    let _ = SHUTDOWN.set(shutdown);
}

pub fn get_shutdown() -> SharedAsyncTaskFactory<()> {
    trace!("Get shutdown");
    SHUTDOWN
        .get_or_init(|| {
            Arc::new(|| {
                Box::pin(async {
                    warn!("Not set shutdown, using default");
                })
            })
        })
        .clone()
}
