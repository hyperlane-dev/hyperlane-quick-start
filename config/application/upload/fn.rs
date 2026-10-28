use super::*;

#[cfg(debug_assertions)]
/// Returns the `UPLOAD_DIR` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn upload_dir() -> &'static str {
    UPLOAD_DIR
}

#[cfg(not(debug_assertions))]
/// Returns the `UPLOAD_DIR` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn upload_dir() -> &'static str {
    UPLOAD_DIR
}

/// Returns the `STATIC_ROUTE` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn static_route() -> &'static str {
    STATIC_ROUTE
}

/// Returns the `UPLOAD_DIR_KEY` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn upload_dir_key() -> &'static str {
    UPLOAD_DIR_KEY
}

/// Returns the `UPLOAD_FILE_KEY` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn upload_file_key() -> &'static str {
    UPLOAD_FILE_KEY
}
