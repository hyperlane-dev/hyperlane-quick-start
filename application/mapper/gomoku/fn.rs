use super::*;

#[instrument_trace]
pub fn get_global_gomoku_rooms() -> &'static ArcRwLock<HashMap<String, GomokuRoom>> {
    GLOBAL_GOMOKU_ROOMS.get_or_init(|| arc_rwlock(HashMap::new()))
}

#[instrument_trace]
pub fn get_global_gomoku_user_rooms() -> &'static ArcRwLock<HashMap<String, String>> {
    GLOBAL_GOMOKU_USER_ROOMS.get_or_init(|| arc_rwlock(HashMap::new()))
}
