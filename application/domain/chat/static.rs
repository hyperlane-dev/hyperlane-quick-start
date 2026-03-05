use super::*;

pub static GLOBAL_CHAT_SESSIONS: OnceLock<ArcRwLock<HashMap<String, ChatSession>>> =
    OnceLock::new();
pub static GLOBAL_ONLINE_USERS: OnceLock<ArcRwLock<HashMap<String, OnlineUser>>> = OnceLock::new();
