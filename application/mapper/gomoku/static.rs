use super::*;

/// global gomoku rooms.
pub static GLOBAL_GOMOKU_ROOMS: OnceLock<ArcRwLock<HashMap<String, GomokuRoom>>> = OnceLock::new();
/// global gomoku user rooms.
pub static GLOBAL_GOMOKU_USER_ROOMS: OnceLock<ArcRwLock<HashMap<String, String>>> = OnceLock::new();
