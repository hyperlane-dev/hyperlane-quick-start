use super::*;

pub static GLOBAL_WEBSOCKET: OnceLock<WebSocket> = OnceLock::new();
pub static GLOBAL_ENV_CONFIG: OnceLock<EnvConfig> = OnceLock::new();
pub static GLOBAL_CHAT_SESSIONS: OnceLock<
    std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, ChatSession>>>,
> = OnceLock::new();
pub static GLOBAL_ONLINE_USERS: OnceLock<
    std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, OnlineUser>>>,
> = OnceLock::new();
