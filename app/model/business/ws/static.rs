use super::*;

pub static GLOBAL_WEBSOCKET: OnceLock<WebSocket> = OnceLock::new();
pub static GLOBAL_ENV_CONFIG: OnceLock<EnvConfig> = OnceLock::new();
