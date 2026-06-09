use super::*;

/// Checks whether the given file extension is in the gzip compressible whitelist.
///
/// Only file types that browsers can correctly decompress and render
/// when served with `Content-Encoding: gzip` should return `true`.
///
/// # Arguments
///
/// - `&str`: The file extension (without leading dot).
///
/// # Returns
///
/// - `bool`: `true` if the extension is gzip-compressible, `false` otherwise.
#[instrument_trace]
pub fn is_gzip_compressible(extension: &str) -> bool {
    GZIP_COMPRESSIBLE_EXTENSIONS.contains(&extension)
}
