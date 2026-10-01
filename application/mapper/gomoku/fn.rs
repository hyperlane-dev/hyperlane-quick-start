use super::*;

/// Returns the global gomoku rooms map, initializing it lazily.
///
/// # Returns
///
/// - `&'static ArcRwLock<HashMap<String, GomokuRoom>>` - The global rooms map.
#[instrument_trace]
pub fn get_global_gomoku_rooms() -> &'static ArcRwLock<HashMap<String, GomokuRoom>> {
    GLOBAL_GOMOKU_ROOMS.get_or_init(|| arc_rwlock(HashMap::new()))
}

/// Returns the global user-to-room index, initializing it lazily.
///
/// # Returns
///
/// - `&'static ArcRwLock<HashMap<String, String>>` - The global user-to-room index.
#[instrument_trace]
pub fn get_global_gomoku_user_rooms() -> &'static ArcRwLock<HashMap<String, String>> {
    GLOBAL_GOMOKU_USER_ROOMS.get_or_init(|| arc_rwlock(HashMap::new()))
}
