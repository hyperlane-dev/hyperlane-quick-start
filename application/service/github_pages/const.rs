/// Error message returned when the owner parameter is empty.
pub const ERROR_OWNER_CANNOT_BE_EMPTY: &str = "Owner cannot be empty";

/// Error message returned when the repository parameter is empty.
pub const ERROR_REPOSITORY_CANNOT_BE_EMPTY: &str = "Repository cannot be empty";

/// Error message returned when fetching GitHub Pages content fails.
pub const ERROR_FAILED_TO_FETCH_GITHUB_PAGES: &str = "Failed to fetch GitHub Pages";

/// Maximum number of retry attempts when fetching a GitHub Pages URL.
pub const GITHUB_PAGES_FETCH_MAX_RETRIES: u32 = 3;

/// File name for the cached index page.
pub const INDEX_HTML_FILE: &str = "index.html";

/// Request timeout in seconds for fetching remote resources.
pub const GITHUB_PAGES_FETCH_TIMEOUT_SECS: u64 = 30;

/// File extensions for which proxy path rewriting should be applied.
///
/// These text-based formats may contain resource references that use the
/// original GitHub Pages path prefix (e.g. `/docs-pages/`) and need to be
/// rewritten to the proxy path prefix (e.g. `/github/pages/{owner}/{repository}/`).
pub const PROXY_REWRITE_EXTENSIONS: &[&str] = &[
    "html", "htm", "css", "js", "mjs", "cjs", "json", "xml", "svg", "txt", "md", "csv", "ics",
    "svgz", "wasm", "map", "scss", "less", "sass", "png", "jpg", "jpeg", "gif", "webp", "bmp",
    "ico", "avif", "tiff", "tif", "woff", "ttf", "otf", "pdf", "yaml", "yml", "toml", "ini",
    "conf", "ts", "tsx", "jsx", "rtf", "log", "sh", "bat", "ps1",
];
