use super::*;
use std::path::PathBuf;
use std::time::SystemTime;
use std::collections::HashMap;

/// 静态文件响应结构
#[derive(Debug, Clone, ToSchema)]
pub struct StaticFileResponse {
    /// 文件内容
    pub content: Vec<u8>,
    /// 内容类型
    pub content_type: String,
    /// 最后修改时间
    #[schema(value_type = String, format = "date-time")]
    pub last_modified: Option<SystemTime>,
    /// ETag
    pub etag: Option<String>,
    /// 文件大小
    pub file_size: u64,
}

impl StaticFileResponse {
    /// 创建新的静态文件响应
    pub fn new(
        content: Vec<u8>,
        content_type: String,
        last_modified: Option<SystemTime>,
        etag: Option<String>,
        file_size: u64,
    ) -> Self {
        Self {
            content,
            content_type,
            last_modified,
            etag,
            file_size,
        }
    }
    
    /// 创建空响应
    pub fn empty() -> Self {
        Self {
            content: Vec::new(),
            content_type: "application/octet-stream".to_string(),
            last_modified: None,
            etag: None,
            file_size: 0,
        }
    }
    
    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
    
    /// 获取内容长度
    pub fn content_length(&self) -> usize {
        self.content.len()
    }
}

/// 资源配置结构
#[derive(Debug, Clone, ToSchema)]
pub struct ResourceConfig {
    /// 资源类型名称
    pub resource_type: String,
    /// 基础目录路径
    pub base_dir: String,
    /// 是否允许目录列表
    pub allow_directory_listing: bool,
    /// 最大文件大小 (字节)
    pub max_file_size: u64,
    /// 缓存策略
    pub cache_strategy: CacheStrategy,
}

impl ResourceConfig {
    /// 创建静态资源配置
    pub fn static_resources() -> Self {
        Self {
            resource_type: "static".to_string(),
            base_dir: "resources/static".to_string(),
            allow_directory_listing: false,
            max_file_size: 100 * 1024 * 1024, // 100MB
            cache_strategy: CacheStrategy::LongTerm,
        }
    }
    
    /// 创建前端资源配置
    pub fn assets() -> Self {
        Self {
            resource_type: "assets".to_string(),
            base_dir: "resources/assets".to_string(),
            allow_directory_listing: false,
            max_file_size: 50 * 1024 * 1024, // 50MB
            cache_strategy: CacheStrategy::LongTerm,
        }
    }
    
    /// 创建上传文件配置
    pub fn uploads() -> Self {
        Self {
            resource_type: "uploads".to_string(),
            base_dir: "uploads".to_string(),
            allow_directory_listing: false,
            max_file_size: 500 * 1024 * 1024, // 500MB
            cache_strategy: CacheStrategy::ShortTerm,
        }
    }
    
    /// 创建公共资源配置
    pub fn public_resources() -> Self {
        Self {
            resource_type: "public".to_string(),
            base_dir: "resources/public".to_string(),
            allow_directory_listing: false,
            max_file_size: 10 * 1024 * 1024, // 10MB
            cache_strategy: CacheStrategy::Custom(3600), // 1小时
        }
    }
    
    /// 获取完整的基础路径
    pub fn get_full_base_path(&self) -> PathBuf {
        PathBuf::from(&self.base_dir)
    }
}

/// 详细文件信息结构
#[derive(Debug, Clone, ToSchema)]
pub struct DetailedFileInfo {
    /// 文件路径
    pub path: String,
    /// 文件大小
    pub file_size: u64,
    /// 最后修改时间
    #[schema(value_type = String, format = "date-time")]
    pub last_modified: SystemTime,
    /// 创建时间
    #[schema(value_type = String, format = "date-time")]
    pub created: SystemTime,
    /// ETag
    pub etag: String,
    /// 内容类型
    pub content_type: String,
    /// 是否可压缩
    pub is_compressible: bool,
    /// 是否可缓存
    pub is_cacheable: bool,
}

/// 流式文件响应结构
#[derive(Debug, ToSchema)]
pub struct StreamFileResponse {
    /// 文件路径
    #[schema(value_type = String)]
    pub file_path: PathBuf,
    /// 开始位置
    pub start: u64,
    /// 结束位置
    pub end: u64,
    /// 内容长度
    pub content_length: u64,
    /// 总文件大小
    pub total_size: u64,
    /// 内容类型
    pub content_type: String,
    /// 最后修改时间
    #[schema(value_type = String, format = "date-time")]
    pub last_modified: Option<SystemTime>,
    /// ETag
    pub etag: Option<String>,
}

/// HTTP 范围请求结构
#[derive(Debug, Clone, ToSchema)]
pub struct HttpRange {
    /// 开始位置
    pub start: usize,
    /// 结束位置
    pub end: usize,
}

/// 文件统计信息
#[derive(Debug, Clone, ToSchema)]
pub struct FileStats {
    /// 总文件数
    pub total_files: u64,
    /// 总大小（字节）
    pub total_size: u64,
    /// 按类型分组的统计
    pub by_type: HashMap<String, TypeStats>,
}

/// 按类型的统计信息
#[derive(Debug, Clone, ToSchema)]
pub struct TypeStats {
    /// 文件数量
    pub count: u64,
    /// 总大小
    pub size: u64,
    /// 平均大小
    pub avg_size: u64,
}

impl TypeStats {
    pub fn new() -> Self {
        Self {
            count: 0,
            size: 0,
            avg_size: 0,
        }
    }
    
    pub fn add_file(&mut self, size: u64) {
        self.count += 1;
        self.size += size;
        self.avg_size = if self.count > 0 { self.size / self.count } else { 0 };
    }
}

/// 资源请求信息
#[derive(Debug, Clone, ToSchema)]
pub struct ResourceRequest {
    /// 资源类型
    pub resource_type: String,
    /// 文件路径
    pub path: String,
    /// 条件请求头
    pub if_modified_since: Option<String>,
    /// ETag 条件请求头
    pub if_none_match: Option<String>,
    /// 范围请求头
    pub range: Option<String>,
}

/// 资源响应信息
#[derive(Debug, Clone, ToSchema)]
pub struct ResourceResponse {
    /// HTTP 状态码
    pub status_code: u16,
    /// 响应头
    pub headers: HashMap<String, String>,
    /// 响应体
    pub body: Option<Vec<u8>>,
    /// 是否为部分内容
    pub is_partial: bool,
}

impl ResourceResponse {
    /// 创建成功响应
    pub fn success(body: Vec<u8>, content_type: String) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), content_type);
        headers.insert("Content-Length".to_string(), body.len().to_string());
        
        Self {
            status_code: 200,
            headers,
            body: Some(body),
            is_partial: false,
        }
    }
    
    /// 创建 304 Not Modified 响应
    pub fn not_modified() -> Self {
        Self {
            status_code: 304,
            headers: HashMap::new(),
            body: None,
            is_partial: false,
        }
    }
    
    /// 创建错误响应
    pub fn error(status_code: u16, message: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain; charset=utf-8".to_string());
        
        Self {
            status_code,
            headers,
            body: Some(message.as_bytes().to_vec()),
            is_partial: false,
        }
    }
    
    /// 创建部分内容响应
    pub fn partial_content(body: Vec<u8>, content_type: String, range: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), content_type);
        headers.insert("Content-Length".to_string(), body.len().to_string());
        headers.insert("Content-Range".to_string(), range.to_string());
        
        Self {
            status_code: 206,
            headers,
            body: Some(body),
            is_partial: true,
        }
    }
}