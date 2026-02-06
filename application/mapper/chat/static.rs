use super::*;

pub static GLOBAL_WEBSOCKET: OnceLock<WebSocket> = OnceLock::new();
pub static GLOBAL_CHAT_SESSIONS: OnceLock<ArcRwLock<HashMap<String, ChatSession>>> =
    OnceLock::new();
pub static GLOBAL_ONLINE_USERS: OnceLock<ArcRwLock<HashMap<String, OnlineUser>>> = OnceLock::new();
pub static ONLINE_CONNECTIONS: &str = "Current number of online connections";
