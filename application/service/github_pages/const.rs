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

/// Separator used in sync task message payloads to delimit owner and repository.
pub const SYNC_TASK_SEPARATOR: &str = "/";

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

/// Maximum number of concurrent resource downloads during sync.
pub const MAX_CONCURRENT_FETCHES: usize = 16;

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
/// Resource file extension for HTML.
pub const RESOURCE_EXTENSION_HTML: &str = "html";

/// Resource file extension for plain CSS.
pub const RESOURCE_EXTENSION_CSS: &str = "css";

/// Resource file extension for SCSS.
pub const RESOURCE_EXTENSION_SCSS: &str = "scss";

/// Resource file extension for Less.
pub const RESOURCE_EXTENSION_LESS: &str = "less";

/// Resource file extension for Sass.
pub const RESOURCE_EXTENSION_SASS: &str = "sass";

/// Resource file extensions whose content is scanned for CSS `url()` and
/// `@import` references.
pub const RESOURCE_STYLESHEET_EXTENSIONS: &[&str] = &[
    RESOURCE_EXTENSION_CSS,
    RESOURCE_EXTENSION_SCSS,
    RESOURCE_EXTENSION_LESS,
    RESOURCE_EXTENSION_SASS,
];

/// Opening token of a CSS `url()` reference.
pub const CSS_URL_FUNCTION_OPEN: &[u8] = b"url(";

/// CSS at-rule importing another stylesheet.
pub const CSS_AT_IMPORT: &[u8] = b"@import";

/// Opening token of an ES `new URL()` call.
pub const ES_NEW_URL_OPEN: &[u8] = b"new URL(";

/// HTML tag name for a script element.
pub const HTML_TAG_SCRIPT: &[u8] = b"script";

/// HTML tag name for a link element.
pub const HTML_TAG_LINK: &[u8] = b"link";

/// HTML tag name for a video element.
pub const HTML_TAG_VIDEO: &[u8] = b"video";

/// HTML tag name for an audio element.
pub const HTML_TAG_AUDIO: &[u8] = b"audio";

/// HTML tag name for a source element.
pub const HTML_TAG_SOURCE: &[u8] = b"source";

/// HTML tag name for a track element.
pub const HTML_TAG_TRACK: &[u8] = b"track";

/// HTML tag name for an embed element.
pub const HTML_TAG_EMBED: &[u8] = b"embed";

/// HTML tag name for an object element.
pub const HTML_TAG_OBJECT: &[u8] = b"object";

/// HTML attribute holding a hyperlink target.
pub const HTML_ATTR_HREF: &[u8] = b"href";

/// HTML attribute holding a linked resource path.
pub const HTML_ATTR_SRC: &[u8] = b"src";

/// Scheme prefix for an inline data URI.
pub const HTML_SCHEME_DATA: &[u8] = b"data";

/// Byte value of the double quote that delimits a quoted value.
pub const BYTE_QUOTE_DOUBLE: u8 = b'"';

/// Byte value of the single quote that delimits a quoted value.
pub const BYTE_QUOTE_SINGLE: u8 = b'\'';

/// Byte value of the closing parenthesis of a CSS `url()` reference.
pub const BYTE_PAREN_CLOSE: u8 = b')';

/// Byte value of the opening angle bracket of an HTML tag.
pub const BYTE_ANGLE_OPEN: u8 = b'<';

/// Byte value of the closing angle bracket of an HTML tag.
pub const BYTE_ANGLE_CLOSE: u8 = b'>';

/// Byte value of the slash that introduces a self-closing or closing tag.
pub const BYTE_SLASH: u8 = b'/';

/// Byte value of the equals sign separating an HTML attribute name and value.
pub const BYTE_EQUALS: u8 = b'=';

/// Byte value of the semicolon terminating an ES module statement.
pub const BYTE_SEMICOLON: u8 = b';';

/// Byte value of the hyphen permitted inside an HTML tag name.
pub const BYTE_HYPHEN: u8 = b'-';

/// Byte value of the lowercase `s` in a closing `</script>` tag.
pub const BYTE_SCRIPT_S_LOWER: u8 = b's';

/// Byte value of the uppercase `S` in a closing `</script>` tag.
pub const BYTE_SCRIPT_S_UPPER: u8 = b'S';

/// Byte value of the lowercase `c` in a closing `</script>` tag.
pub const BYTE_SCRIPT_C_LOWER: u8 = b'c';

/// Byte value of the uppercase `C` in a closing `</script>` tag.
pub const BYTE_SCRIPT_C_UPPER: u8 = b'C';

/// Byte value of the lowercase `r` in a closing `</script>` tag.
pub const BYTE_SCRIPT_R_LOWER: u8 = b'r';

/// Byte value of the uppercase `R` in a closing `</script>` tag.
pub const BYTE_SCRIPT_R_UPPER: u8 = b'R';

/// Byte value of the lowercase `i` in a closing `</script>` tag.
pub const BYTE_SCRIPT_I_LOWER: u8 = b'i';

/// Byte value of the uppercase `I` in a closing `</script>` tag.
pub const BYTE_SCRIPT_I_UPPER: u8 = b'I';

/// Byte value of the lowercase `p` in a closing `</script>` tag.
pub const BYTE_SCRIPT_P_LOWER: u8 = b'p';

/// Byte value of the uppercase `P` in a closing `</script>` tag.
pub const BYTE_SCRIPT_P_UPPER: u8 = b'P';

/// Byte value of the lowercase `t` in a closing `</script>` tag.
pub const BYTE_SCRIPT_T_LOWER: u8 = b't';

/// Byte value of the uppercase `T` in a closing `</script>` tag.
pub const BYTE_SCRIPT_T_UPPER: u8 = b'T';

/// ES module import keyword.
pub const ES_IMPORT_KEYWORD: &[u8] = b"import";

/// ES module from keyword.
pub const ES_FROM_KEYWORD: &[u8] = b"from";

/// Plain HTTP URL scheme prefix.
pub const URL_SCHEME_HTTP: &str = "http://";

/// Secure HTTP URL scheme prefix.
pub const URL_SCHEME_HTTPS: &str = "https://";

/// Data URI scheme prefix.
pub const URL_SCHEME_DATA: &str = "data:";

/// Error when the shared HTTP client cannot be created.
pub const ERROR_SHARED_HTTP_CLIENT: &str = "Failed to create shared HTTP client";

/// Error when a pending fetch is cancelled.
pub const ERROR_FETCH_CANCELLED: &str = "Pending fetch was cancelled";

/// Route placeholder for the repository owner.
pub const PLACEHOLDER_OWNER: &str = "{owner}";

/// Route placeholder for the repository name.
pub const PLACEHOLDER_REPOSITORY: &str = "{repository}";

/// Date and time format used in cache keys.
pub const FORMAT_DATE_TIME: &str = "%Y-%m-%d %H:%M:%S";
