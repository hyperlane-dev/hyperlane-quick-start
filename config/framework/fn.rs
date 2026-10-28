use super::*;

/// Returns the `DEFAULT_CACHE_CONTROL_STATIC_ASSETS` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn default_cache_control_static_assets() -> &'static str {
    DEFAULT_CACHE_CONTROL_STATIC_ASSETS
}

/// Returns the `DEFAULT_EXPIRES_FAR_FUTURE` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn default_expires_far_future() -> &'static str {
    DEFAULT_EXPIRES_FAR_FUTURE
}
