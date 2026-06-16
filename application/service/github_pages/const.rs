/// Error message returned when the owner parameter is empty.
pub const ERROR_OWNER_CANNOT_BE_EMPTY: &str = "Owner cannot be empty";

/// Prefix used in HTTP Range request headers (e.g. `Range: bytes=0-1023`).
pub const RANGE_HEADER_PREFIX: &str = "bytes=";

/// Error message returned when the Range header format is invalid.
pub const ERROR_INVALID_RANGE_HEADER_FORMAT: &str = "Invalid range header format";

/// Error message returned when the Range header specification is malformed.
pub const ERROR_INVALID_RANGE_SPECIFICATION: &str = "Invalid range specification";

/// Error message returned when the range start exceeds the file size.
pub const ERROR_RANGE_START_EXCEEDS_FILE_SIZE: &str = "Range start exceeds file size";

/// Error message returned when the range start is greater than the range end.
pub const ERROR_INVALID_RANGE_START_GREATER_THAN_END: &str = "Invalid range: start > end";

/// Error message returned when the repository parameter is empty.
pub const ERROR_REPOSITORY_CANNOT_BE_EMPTY: &str = "Repository cannot be empty";

/// Error message returned when fetching GitHub Pages content fails.
pub const ERROR_FAILED_TO_FETCH_GITHUB_PAGES: &str = "Failed to fetch GitHub Pages";

/// Success message returned when GitHub Pages sync completes.
pub const SUCCESS_GITHUB_PAGES_SYNCED: &str = "Synced";

/// Error message returned when a path contains unsafe traversal characters.
pub const ERROR_UNSAFE_PATH: &str = "Unsafe path detected";

/// Maximum number of retry attempts when fetching a GitHub Pages URL.
pub const FETCH_MAX_RETRIES: u32 = 3;

/// File name for the cached index page.
pub const INDEX_HTML_FILE: &str = "index.html";

/// Request timeout in seconds for fetching remote resources.
///
/// Set to 600 seconds (10 minutes) to accommodate large files (e.g. video) that may
/// take longer to download on slower connections.
pub const FETCH_TIMEOUT_SECS: u64 = 600;

/// Maximum number of redirects to follow.
pub const MAX_REDIRECTS: usize = 8;

/// File extensions for which proxy path rewriting should be applied.
///
/// Only text-based formats that may contain resource references using the
/// original GitHub Pages path prefix (e.g. `/docs-pages/`) are included.
/// Binary formats (images, fonts, media, etc.) are excluded to avoid content corruption.
pub const PROXY_REWRITE_EXTENSIONS: &[&str] = &[
    "html", "htm", "css", "js", "mjs", "cjs", "json", "xml", "svg", "txt", "md", "csv", "ics",
    "map", "scss", "less", "sass", "yaml", "yml", "toml", "ini", "conf", "ts", "tsx", "jsx", "rtf",
    "log", "sh", "bat", "ps1",
];

/// File extensions for which linked resource path extraction should be applied.
///
/// Includes all text-based formats that may reference other resources (HTML, JS, CSS)
/// plus media formats that may be referenced by HTML tags (video, audio, images, fonts, etc.).
/// This ensures that media resources referenced in HTML pages are discovered during sync.
pub const RESOURCE_LINK_EXTENSIONS: &[&str] = &[
    "html", "htm", "css", "js", "mjs", "cjs", "json", "xml", "svg", "txt", "md", "csv", "ics",
    "map", "scss", "less", "sass", "yaml", "yml", "toml", "ini", "conf", "ts", "tsx", "jsx", "rtf",
    "log", "sh", "bat", "ps1", "mp4", "mp3", "webm", "ogg", "wav", "flac", "m4a", "m4v", "avi",
    "mov", "wmv", "webp", "png", "jpg", "jpeg", "gif", "bmp", "ico", "avif", "tiff", "tif", "woff",
    "woff2", "ttf", "otf", "eot", "pdf", "wasm", "swf",
];

/// File extensions that typically require HTTP Range request support for streaming.
///
/// Video and audio formats where browsers send Range requests for seeking/buffering.
pub const STREAMABLE_EXTENSIONS: &[&str] = &[
    "mp4", "webm", "ogg", "m4v", "avi", "mov", "wmv", "flv", "mkv", "mp3", "wav", "flac", "m4a",
    "aac", "oga", "pdf",
];
