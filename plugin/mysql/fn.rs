use super::*;

/// Returns the `DEFAULT_MYSQL_INSTANCE_NAME` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn default_mysql_instance_name() -> &'static str {
    DEFAULT_MYSQL_INSTANCE_NAME
}
