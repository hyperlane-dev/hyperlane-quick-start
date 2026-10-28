use super::*;

/// Returns the `DEFAULT_POSTGRESQL_INSTANCE_NAME` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn default_postgresql_instance_name() -> &'static str {
    DEFAULT_POSTGRESQL_INSTANCE_NAME
}
