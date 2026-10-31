use super::*;

/// Returns the `STATIC_RESOURCES_DIR` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn static_resources_dir() -> &'static str {
    STATIC_RESOURCES_DIR
}

/// Returns the `PATH_KEY` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn path_key() -> &'static str {
    PATH_KEY
}

/// Returns the `GZIP_COMPRESSIBLE_EXTENSIONS` configuration value.
///
/// # Returns
///
/// - `&'static [&'static str]` - The configured value.
pub fn gzip_compressible_extensions() -> &'static [&'static str] {
    GZIP_COMPRESSIBLE_EXTENSIONS
}
