pub const ERROR_OWNER_CANNOT_BE_EMPTY: &str = "Owner cannot be empty";
pub const ERROR_REPOSITORY_CANNOT_BE_EMPTY: &str = "Repository cannot be empty";
pub const ERROR_INVALID_OWNER_REPOSITORY_FORMAT: &str = "Invalid format, expected owner/repository";
pub const ERROR_FAILED_TO_FETCH_GITHUB_PAGES: &str = "Failed to fetch GitHub Pages";
pub const ERROR_FAILED_TO_PARSE_HTML: &str = "Failed to parse HTML";
pub const ERROR_FAILED_TO_CREATE_DIRECTORY: &str = "Failed to create directory";
pub const ERROR_FAILED_TO_WRITE_FILE: &str = "Failed to write file";
pub const GITHUB_PAGES_FETCH_MAX_RETRIES: u32 = 8;
pub const GITHUB_PAGES_STATIC_EXTENSIONS: &[&str] = &[
    ".css",
    ".js",
    ".png",
    ".jpg",
    ".jpeg",
    ".gif",
    ".svg",
    ".ico",
    ".woff",
    ".woff2",
    ".ttf",
    ".eot",
    ".json",
    ".webp",
    ".webmanifest",
    ".wasm",
];
pub const GITHUB_PAGES_HTML_EXTENSIONS: &[&str] = &[".html", ".htm"];
pub const GITHUB_PAGES_RESOURCE_ATTR_CONFIGS: &[(&str, &str)] = &[
    ("script[src]", "src"),
    ("img[src]", "src"),
    ("link[href]", "href"),
    ("source[src]", "src"),
    ("video[src]", "src"),
    ("video[poster]", "poster"),
    ("audio[src]", "src"),
    ("track[src]", "src"),
    ("iframe[src]", "src"),
    ("embed[src]", "src"),
    ("object[data]", "data"),
    ("input[src]", "src"),
    ("[data-src]", "data-src"),
];
