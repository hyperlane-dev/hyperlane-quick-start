use super::*;

#[cfg(debug_assertions)]
/// Returns the `LOG_LEVEL_FILTER` configuration value.
///
/// # Returns
///
/// - `LevelFilter` - The configured value.
pub fn log_level_filter() -> LevelFilter {
    LOG_LEVEL_FILTER
}

#[cfg(not(debug_assertions))]
/// Returns the `LOG_LEVEL_FILTER` configuration value.
///
/// # Returns
///
/// - `LevelFilter` - The configured value.
pub fn log_level_filter() -> LevelFilter {
    LOG_LEVEL_FILTER
}
