use super::*;

pub(super) static SHUTDOWN: OnceLock<SharedAsyncTaskFactory<()>> = OnceLock::new();
