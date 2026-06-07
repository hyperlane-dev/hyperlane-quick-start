use super::*;

/// get global gomoku rooms.
#[instrument_trace]
pub fn get_global_gomoku_rooms() -> &'static ArcRwLock<HashMap<String, GomokuRoom>> {
    GLOBAL_GOMOKU_ROOMS.get_or_init(|| arc_rwlock(HashMap::new()))
}

/// get global gomoku user rooms.
#[instrument_trace]
pub fn get_global_gomoku_user_rooms() -> &'static ArcRwLock<HashMap<String, String>> {
    GLOBAL_GOMOKU_USER_ROOMS.get_or_init(|| arc_rwlock(HashMap::new()))
}
