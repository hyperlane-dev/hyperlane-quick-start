/// Directory path for static resources dir.
pub const STATIC_RESOURCES_DIR: &str = "./resources/static";

/// Key for path key.
pub const PATH_KEY: &str = "path";

/// Cache or TTL value for control static assets.
pub const CACHE_CONTROL_STATIC_ASSETS: &str = "public, max-age=31536000";

/// Expires far future.
pub const EXPIRES_FAR_FUTURE: &str = "Thu, 31 Dec 2037 23:55:55 GMT";

/// File extensions that are safe to serve with gzip Content-Encoding,
/// meaning browsers can correctly decompress and render the content.
pub const GZIP_COMPRESSIBLE_EXTENSIONS: &[&str] = &[
    "html", "htm", "css", "js", "mjs", "cjs", "json", "xml", "svg", "txt", "md", "csv", "ics",
    "svgz", "wasm", "map", "scss", "less", "sass", "png", "jpg", "jpeg", "gif", "webp", "bmp",
    "ico", "avif", "tiff", "tif", "woff", "ttf", "otf", "pdf", "yaml", "yml", "toml", "ini",
    "conf", "ts", "tsx", "jsx", "rtf", "log", "sh", "bat", "ps1",
];
