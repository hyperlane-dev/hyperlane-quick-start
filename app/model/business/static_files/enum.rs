use super::*;

/// 静态文件错误枚举
#[derive(Debug, Clone, ToSchema)]
pub enum StaticFileError {
    /// 文件未找到
    NotFound,
    /// 访问被禁止
    Forbidden,
    /// 安全违规（路径遍历等）
    SecurityViolation,
    /// 文件过大
    FileTooLarge,
    /// 无效路径
    InvalidPath,
    /// IO 错误
    IoError(String),
}

impl std::fmt::Display for StaticFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StaticFileError::NotFound => write!(f, "File not found"),
            StaticFileError::Forbidden => write!(f, "Access forbidden"),
            StaticFileError::SecurityViolation => write!(f, "Security violation detected"),
            StaticFileError::FileTooLarge => write!(f, "File too large"),
            StaticFileError::InvalidPath => write!(f, "Invalid file path"),
            StaticFileError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for StaticFileError {}

impl From<std::io::Error> for StaticFileError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => StaticFileError::NotFound,
            std::io::ErrorKind::PermissionDenied => StaticFileError::Forbidden,
            _ => StaticFileError::IoError(error.to_string()),
        }
    }
}

/// 安全错误枚举
#[derive(Debug, Clone, ToSchema)]
pub enum SecurityError {
    /// 路径遍历攻击
    PathTraversal,
    /// 无效路径
    InvalidPath,
    /// 超出边界
    OutOfBounds,
    /// 无效基础路径
    InvalidBasePath,
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityError::PathTraversal => write!(f, "Path traversal attempt detected"),
            SecurityError::InvalidPath => write!(f, "Invalid path"),
            SecurityError::OutOfBounds => write!(f, "Path out of bounds"),
            SecurityError::InvalidBasePath => write!(f, "Invalid base path"),
        }
    }
}

impl std::error::Error for SecurityError {}

impl From<SecurityError> for StaticFileError {
    fn from(error: SecurityError) -> Self {
        match error {
            SecurityError::PathTraversal => StaticFileError::SecurityViolation,
            SecurityError::OutOfBounds => StaticFileError::Forbidden,
            _ => StaticFileError::InvalidPath,
        }
    }
}

/// 缓存策略枚举
#[derive(Debug, Clone, ToSchema)]
pub enum CacheStrategy {
    /// 长期缓存 (1年)
    LongTerm,
    /// 短期缓存 (1小时)
    ShortTerm,
    /// 无缓存
    NoCache,
    /// 自定义缓存时间 (秒)
    Custom(u32),
}

impl CacheStrategy {
    /// 获取缓存时间 (秒)
    pub fn get_max_age(&self) -> u32 {
        match self {
            CacheStrategy::LongTerm => 31536000, // 1年
            CacheStrategy::ShortTerm => 3600,    // 1小时
            CacheStrategy::NoCache => 0,
            CacheStrategy::Custom(seconds) => *seconds,
        }
    }

    /// 获取 Cache-Control 头值
    pub fn get_cache_control(&self) -> String {
        match self {
            CacheStrategy::NoCache => "no-cache, no-store, must-revalidate".to_string(),
            _ => format!("public, max-age={}", self.get_max_age()),
        }
    }
}

/// 资源类型枚举
#[derive(Debug, Clone, ToSchema)]
pub enum ResourceType {
    /// 静态资源
    Static,
    /// 前端资源
    Assets,
    /// 上传文件
    Uploads,
    /// 公共资源
    Public,
}

impl ResourceType {
    /// 从字符串解析资源类型
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "static" => Some(ResourceType::Static),
            "assets" => Some(ResourceType::Assets),
            "uploads" => Some(ResourceType::Uploads),
            "public" => Some(ResourceType::Public),
            _ => None,
        }
    }

    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceType::Static => "static",
            ResourceType::Assets => "assets",
            ResourceType::Uploads => "uploads",
            ResourceType::Public => "public",
        }
    }

    /// 获取基础目录
    pub fn get_base_dir(&self) -> &'static str {
        match self {
            ResourceType::Static => "resources/static",
            ResourceType::Assets => "resources/assets",
            ResourceType::Uploads => "uploads",
            ResourceType::Public => "resources/public",
        }
    }

    /// 获取最大文件大小
    pub fn get_max_file_size(&self) -> u64 {
        match self {
            ResourceType::Static => 100 * 1024 * 1024,  // 100MB
            ResourceType::Assets => 50 * 1024 * 1024,   // 50MB
            ResourceType::Uploads => 500 * 1024 * 1024, // 500MB
            ResourceType::Public => 10 * 1024 * 1024,   // 10MB
        }
    }

    /// 获取缓存策略
    pub fn get_cache_strategy(&self) -> CacheStrategy {
        match self {
            ResourceType::Static => CacheStrategy::LongTerm,
            ResourceType::Assets => CacheStrategy::LongTerm,
            ResourceType::Uploads => CacheStrategy::ShortTerm,
            ResourceType::Public => CacheStrategy::Custom(3600),
        }
    }
}
