use super::*;

pub static GLOBAL_GOMOKU_WEBSOCKET: OnceLock<WebSocket> = OnceLock::new();
pub static GLOBAL_GOMOKU_ROOMS: OnceLock<Arc<RwLock<HashMap<String, GomokuRoom>>>> =
    OnceLock::new();
pub static GLOBAL_GOMOKU_USER_ROOMS: OnceLock<Arc<RwLock<HashMap<String, String>>>> =
    OnceLock::new();
