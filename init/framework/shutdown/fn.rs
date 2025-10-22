use super::*;

pub fn set_shutdown(shutdown: SharedAsyncTaskFactory<()>) {
    let _: Result<(), SharedAsyncTaskFactory<()>> = SHUTDOWN.set(shutdown);
}

pub fn shutdown() -> SharedAsyncTaskFactory<()> {
    SHUTDOWN
        .get_or_init(|| Arc::new(|| Box::pin(async {})))
        .clone()
}
