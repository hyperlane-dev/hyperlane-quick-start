use super::*;

/// Formats the `Content-Type` header value, appending `charset=utf-8` for
/// text-based content types and leaving binary types unchanged.
///
/// Checks whether the given file extension is in `TEXT_CONTENT_EXTENSIONS`.
/// If so, calls `ContentType::format_content_type_with_charset` with `UTF8`;
/// otherwise returns the original content type string unchanged.
///
/// # Arguments
/// - `&str`: The raw content type string (e.g. `"text/html"`, `"video/mp4"`).
/// - `&str`: The file extension (without leading dot), e.g. `"html"`, `"mp4"`.
///
/// # Returns
/// - `String`: The formatted content type, with charset appended for text types.
#[instrument_trace]
pub fn format_content_type(content_type: &str, extension: &str) -> String {
    if TEXT_CONTENT_EXTENSIONS.contains(&extension) {
        ContentType::format_content_type_with_charset(content_type, UTF8)
    } else {
        content_type.to_string()
    }
}
