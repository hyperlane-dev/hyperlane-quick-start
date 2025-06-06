use super::*;

pub static GLOBAL_WEBSOCKET: OnceLock<WebSocket> = OnceLock::new();
