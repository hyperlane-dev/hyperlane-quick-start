use super::*;

pub(super) static SHUTDOWN: OnceLock<ServerControlHookHandler<()>> = OnceLock::new();
