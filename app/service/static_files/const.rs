/// 静态文件服务相关常量

/// 默认静态资源目录
pub const DEFAULT_STATIC_DIR: &str = "resources/static";

/// 默认资产目录
pub const DEFAULT_ASSETS_DIR: &str = "resources/assets";

/// 默认上传目录
pub const DEFAULT_UPLOADS_DIR: &str = "uploads";

/// 默认公共资源目录
pub const DEFAULT_PUBLIC_DIR: &str = "resources/public";

/// 最大文件大小限制
pub const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB

/// 上传文件最大大小限制
pub const MAX_UPLOAD_SIZE: u64 = 500 * 1024 * 1024; // 500MB

/// 小文件最大大小限制
pub const MAX_SMALL_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB

/// 缓存最大时间（秒）
pub const CACHE_MAX_AGE: u32 = 31536000; // 1年

/// 短期缓存时间（秒）
pub const CACHE_SHORT_TERM: u32 = 3600; // 1小时

/// 中期缓存时间（秒）
pub const CACHE_MEDIUM_TERM: u32 = 86400; // 1天

/// 长期缓存时间（秒）
pub const CACHE_LONG_TERM: u32 = 604800; // 1周

/// 静态资源缓存控制头
pub const CACHE_CONTROL_STATIC_RESOURCES: &str = "public, max-age=31536000, immutable";

/// HTML 文件缓存控制头
pub const CACHE_CONTROL_HTML: &str = "public, max-age=3600, must-revalidate";

/// JSON 文件缓存控制头
pub const CACHE_CONTROL_JSON: &str = "public, max-age=86400";

/// 无缓存控制头
pub const CACHE_CONTROL_NO_CACHE: &str = "no-cache, no-store, must-revalidate";

/// 路径参数键名
pub const STATIC_PATH_KEY: &str = "path";

/// 资源类型参数键名
pub const RESOURCE_TYPE_KEY: &str = "resource_type";

/// 错误消息常量
pub const ERROR_FILE_NOT_FOUND: &str = "File not found";
pub const ERROR_PATH_TRAVERSAL: &str = "Path traversal attack detected";
pub const ERROR_INVALID_PATH: &str = "Invalid file path";
pub const ERROR_ACCESS_DENIED: &str = "Access denied";
pub const ERROR_FILE_TOO_LARGE: &str = "File too large";
pub const ERROR_INVALID_RESOURCE_TYPE: &str = "Invalid resource type";

/// 支持的资源类型
pub const SUPPORTED_RESOURCE_TYPES: &[&str] = &["static", "assets", "uploads", "public"];

/// 默认 MIME 类型
pub const DEFAULT_MIME_TYPE: &str = "application/octet-stream";

/// 文本 MIME 类型前缀
pub const TEXT_MIME_PREFIX: &str = "text/";

/// 图片 MIME 类型前缀
pub const IMAGE_MIME_PREFIX: &str = "image/";

/// 音频 MIME 类型前缀
pub const AUDIO_MIME_PREFIX: &str = "audio/";

/// 视频 MIME 类型前缀
pub const VIDEO_MIME_PREFIX: &str = "video/";

/// 字体 MIME 类型前缀
pub const FONT_MIME_PREFIX: &str = "font/";

/// 应用程序 MIME 类型前缀
pub const APPLICATION_MIME_PREFIX: &str = "application/";

/// 可压缩的 MIME 类型
pub const COMPRESSIBLE_MIME_TYPES: &[&str] = &[
    "text/",
    "application/javascript",
    "application/json",
    "application/xml",
    "image/svg+xml",
    "application/manifest+json",
];

/// 需要字符集的 MIME 类型
pub const CHARSET_MIME_TYPES: &[&str] = &[
    "text/",
    "application/javascript",
    "application/json",
    "application/xml",
    "image/svg+xml",
];

/// 默认字符集
pub const DEFAULT_CHARSET: &str = "utf-8";

/// HTTP 日期格式
pub const HTTP_DATE_FORMAT: &str = "%a, %d %b %Y %H:%M:%S GMT";

/// 备用 HTTP 日期格式
pub const HTTP_DATE_FORMAT_RFC850: &str = "%A, %d-%b-%y %H:%M:%S GMT";

/// ANSI C 日期格式
pub const HTTP_DATE_FORMAT_ANSI: &str = "%a %b %d %H:%M:%S %Y";

/// 最大路径长度
pub const MAX_PATH_LENGTH: usize = 255;

/// 最大文件名长度
pub const MAX_FILENAME_LENGTH: usize = 255;

/// 危险字符列表
pub const DANGEROUS_CHARS: &[char] = &['<', '>', ':', '"', '|', '?', '*', '\0'];

/// Windows 保留文件名
pub const WINDOWS_RESERVED_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL",
    "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
    "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"
];

/// 隐藏文件前缀
pub const HIDDEN_FILE_PREFIX: &str = ".";

/// 路径分隔符
pub const PATH_SEPARATOR: &str = "/";

/// Windows 路径分隔符
pub const WINDOWS_PATH_SEPARATOR: &str = "\\";

/// 父目录标识符
pub const PARENT_DIR: &str = "..";

/// 当前目录标识符
pub const CURRENT_DIR: &str = ".";

/// ETag 弱标识符前缀
pub const ETAG_WEAK_PREFIX: &str = "W/";

/// ETag 引号
pub const ETAG_QUOTE: &str = "\"";

/// HTTP 范围请求前缀
pub const RANGE_BYTES_PREFIX: &str = "bytes=";

/// 范围分隔符
pub const RANGE_SEPARATOR: &str = "-";

/// 范围列表分隔符
pub const RANGE_LIST_SEPARATOR: &str = ",";

/// 默认缓存清理间隔（秒）
pub const DEFAULT_CACHE_CLEANUP_INTERVAL: u64 = 86400; // 1天

/// 默认缓存过期时间（秒）
pub const DEFAULT_CACHE_EXPIRY: u64 = 604800; // 1周

/// 流式传输块大小
pub const STREAM_CHUNK_SIZE: usize = 8192; // 8KB

/// 大文件阈值
pub const LARGE_FILE_THRESHOLD: u64 = 10 * 1024 * 1024; // 10MB

/// 并发文件处理限制
pub const MAX_CONCURRENT_FILES: usize = 100;

/// 文件监控间隔（毫秒）
pub const FILE_WATCH_INTERVAL: u64 = 1000; // 1秒

/// 统计信息更新间隔（秒）
pub const STATS_UPDATE_INTERVAL: u64 = 300; // 5分钟