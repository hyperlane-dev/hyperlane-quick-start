/// 统一静态资源配置常量

// 资源目录配置
pub const STATIC_RESOURCES_DIR: &str = "resources/static";
pub const ASSETS_DIR: &str = "resources/assets";
pub const UPLOADS_DIR: &str = "uploads";
pub const PUBLIC_DIR: &str = "resources/public";

// 路由前缀
pub const STATIC_ROUTE_PREFIX: &str = "static";
pub const ASSETS_ROUTE_PREFIX: &str = "assets";
pub const UPLOADS_ROUTE_PREFIX: &str = "uploads";
pub const PUBLIC_ROUTE_PREFIX: &str = "public";

// 文件大小限制
pub const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB
pub const MAX_UPLOAD_SIZE: u64 = 500 * 1024 * 1024; // 500MB
pub const MAX_ASSET_SIZE: u64 = 50 * 1024 * 1024; // 50MB
pub const MAX_PUBLIC_SIZE: u64 = 10 * 1024 * 1024; // 10MB

// 缓存配置
pub const CACHE_MAX_AGE: u32 = 31536000; // 1年
pub const CACHE_SHORT_TERM: u32 = 3600; // 1小时
pub const CACHE_MEDIUM_TERM: u32 = 86400; // 1天
pub const CACHE_LONG_TERM: u32 = 604800; // 1周

// 路由参数键名
pub const STATIC_PATH_KEY: &str = "path";
pub const RESOURCE_TYPE_KEY: &str = "resource_type";

// HTTP 头部常量
pub const IF_MODIFIED_SINCE: &str = "If-Modified-Since";
pub const IF_NONE_MATCH: &str = "If-None-Match";
pub const LAST_MODIFIED: &str = "Last-Modified";
pub const ETAG: &str = "ETag";
pub const CACHE_CONTROL: &str = "Cache-Control";
pub const CONTENT_TYPE: &str = "Content-Type";
pub const CONTENT_LENGTH: &str = "Content-Length";
pub const CONTENT_RANGE: &str = "Content-Range";
pub const ACCEPT_RANGES: &str = "Accept-Ranges";
pub const EXPIRES: &str = "Expires";
pub const RANGE: &str = "Range";

// 缓存控制头值
pub const CACHE_CONTROL_STATIC_RESOURCES: &str = "public, max-age=31536000, immutable";
pub const CACHE_CONTROL_HTML: &str = "public, max-age=3600, must-revalidate";
pub const CACHE_CONTROL_JSON: &str = "public, max-age=86400";
pub const CACHE_CONTROL_NO_CACHE: &str = "no-cache, no-store, must-revalidate";

// 错误消息
pub const ERROR_FILE_NOT_FOUND: &str = "File not found";
pub const ERROR_PATH_TRAVERSAL: &str = "Path traversal attempt detected";
pub const ERROR_INVALID_PATH: &str = "Invalid file path";
pub const ERROR_ACCESS_DENIED: &str = "Access denied";
pub const ERROR_FILE_TOO_LARGE: &str = "File too large";
pub const ERROR_INVALID_RESOURCE_TYPE: &str = "Invalid resource type";
pub const ERROR_RANGE_NOT_SATISFIABLE: &str = "Range not satisfiable";

// 支持的资源类型
pub const SUPPORTED_RESOURCE_TYPES: &[&str] = &["static", "assets", "uploads", "public"];

// MIME 类型常量
pub const DEFAULT_MIME_TYPE: &str = "application/octet-stream";
pub const TEXT_PLAIN_UTF8: &str = "text/plain; charset=utf-8";
pub const TEXT_HTML_UTF8: &str = "text/html; charset=utf-8";
pub const APPLICATION_JSON_UTF8: &str = "application/json; charset=utf-8";

// 安全配置
pub const MAX_PATH_LENGTH: usize = 255;
pub const MAX_FILENAME_LENGTH: usize = 255;
pub const DANGEROUS_CHARS: &[char] = &['<', '>', ':', '"', '|', '?', '*', '\0'];
pub const HIDDEN_FILE_PREFIX: &str = ".";

// 性能配置
pub const STREAM_CHUNK_SIZE: usize = 8192; // 8KB
pub const LARGE_FILE_THRESHOLD: u64 = 10 * 1024 * 1024; // 10MB
pub const MAX_CONCURRENT_FILES: usize = 100;

// 统计和监控
pub const STATS_UPDATE_INTERVAL: u64 = 300; // 5分钟
pub const CACHE_CLEANUP_INTERVAL: u64 = 86400; // 1天
