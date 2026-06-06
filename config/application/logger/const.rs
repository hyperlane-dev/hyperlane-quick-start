use super::*;

/// Log level filter for debug builds, enabling trace-level logging.
#[cfg(debug_assertions)]
pub const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Trace;
/// Log level filter for release builds, enabling info-level logging and above.
#[cfg(not(debug_assertions))]
pub const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Info;
