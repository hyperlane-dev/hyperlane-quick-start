use super::*;

/// Returns the `DEFAULT_REDIS_INSTANCE_NAME` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn default_redis_instance_name() -> &'static str {
    DEFAULT_REDIS_INSTANCE_NAME
}
