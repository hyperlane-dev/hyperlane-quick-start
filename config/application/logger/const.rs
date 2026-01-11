use super::*;

#[cfg(debug_assertions)]
pub const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Debug;
#[cfg(not(debug_assertions))]
pub const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Info;
