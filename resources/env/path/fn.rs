use super::*;

#[cfg(debug_assertions)]
/// Returns the `SERVER_ENV_FILE_PATH` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn server_env_file_path() -> &'static str {
    SERVER_ENV_FILE_PATH
}

#[cfg(not(debug_assertions))]
/// Returns the `SERVER_ENV_FILE_PATH` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn server_env_file_path() -> &'static str {
    SERVER_ENV_FILE_PATH
}
