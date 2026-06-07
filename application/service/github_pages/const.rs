/// Error message returned when the owner parameter is empty.
pub const ERROR_OWNER_CANNOT_BE_EMPTY: &str = "Owner cannot be empty";

/// Error message returned when the repository parameter is empty.
pub const ERROR_REPOSITORY_CANNOT_BE_EMPTY: &str = "Repository cannot be empty";

/// Error message returned when the owner/repository format is invalid.
pub const ERROR_INVALID_OWNER_REPOSITORY_FORMAT: &str = "Invalid format, expected owner/repository";

/// Error message returned when fetching GitHub Pages content fails.
pub const ERROR_FAILED_TO_FETCH_GITHUB_PAGES: &str = "Failed to fetch GitHub Pages";

/// Error message returned when creating the cache directory fails.
pub const ERROR_FAILED_TO_CREATE_DIRECTORY: &str = "Failed to create directory";

/// Error message returned when writing a file to the cache fails.
pub const ERROR_FAILED_TO_WRITE_FILE: &str = "Failed to write file";

/// Maximum number of retry attempts when fetching a GitHub Pages URL.
pub const GITHUB_PAGES_FETCH_MAX_RETRIES: u32 = 8;

/// File name for the cached index page.
pub const INDEX_HTML_FILE: &str = "index.html";

/// File extensions considered as text content that may contain references to other resources.
pub const TEXT_CONTENT_EXTENSIONS: &[&str] = &[
    "html", "htm", "js", "mjs", "cjs", "css", "scss", "less", "sass", "svg", "xml", "json", "map",
    "wasm",
];
