use super::*;

pub static GLOBAL_GOMOKU_ROOMS: OnceLock<ArcRwLock<HashMap<String, GomokuRoom>>> = OnceLock::new();
pub static GLOBAL_GOMOKU_USER_ROOMS: OnceLock<ArcRwLock<HashMap<String, String>>> = OnceLock::new();
