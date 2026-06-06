use super::*;

/// Global static storage for the shutdown hook handler, initialized once.
pub(super) static SHUTDOWN: OnceLock<ServerControlHookHandler<()>> = OnceLock::new();
