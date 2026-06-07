use super::*;

/// Global singleton WebSocket service instance, initialized once at startup.
pub static GLOBAL_WEBSOCKET: OnceLock<WebSocket> = OnceLock::new();
