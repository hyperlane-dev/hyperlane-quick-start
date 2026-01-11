use super::*;

#[cfg(debug_assertions)]
pub const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Trace;
#[cfg(not(debug_assertions))]
pub const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Info;
