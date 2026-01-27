use super::*;

#[instrument_trace]
pub fn get_global_gomoku_websocket() -> &'static WebSocket {
    GLOBAL_GOMOKU_WEBSOCKET.get_or_init(WebSocket::new)
}

#[instrument_trace]
pub fn get_global_gomoku_rooms() -> &'static Arc<RwLock<HashMap<String, GomokuRoom>>> {
    GLOBAL_GOMOKU_ROOMS.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

#[instrument_trace]
pub fn get_global_gomoku_user_rooms() -> &'static Arc<RwLock<HashMap<String, String>>> {
    GLOBAL_GOMOKU_USER_ROOMS.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

