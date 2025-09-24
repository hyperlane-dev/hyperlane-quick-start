use super::*;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// 验证统一路径安全性
pub fn validate_unified_path_security(
    config: &ResourceConfig,
    path: &str,
) -> Result<PathBuf, SecurityError> {
    // 1. 检查是否包含危险字符
    if path.contains("../") || path.contains("..\\") {
        return Err(SecurityError::PathTraversal);
    }
    
    // 2. 检查是否包含绝对路径
    if path.starts_with('/') || path.starts_with('\\') || path.contains(':') {
        return Err(SecurityError::PathTraversal);
    }
    
    // 3. 检查是否包含隐藏文件或目录（以 . 开头的文件/目录）
    // 但不包括正常的文件扩展名
    for segment in path.split('/') {
        if segment.starts_with('.') && segment != "." && segment != ".." {
            return Err(SecurityError::InvalidPath);
        }
    }
    
    // 4. 检查路径长度
    if path.len() > 255 {
        return Err(SecurityError::InvalidPath);
    }
    
    // 5. 检查是否为空路径
    if path.is_empty() {
        return Err(SecurityError::InvalidPath);
    }
    
    // 6. 构建完整路径
    let base_path = Path::new(&config.base_dir);
    let full_path = base_path.join(path);
    
    // 7. 规范化路径
    let canonical_path = match full_path.canonicalize() {
        Ok(path) => path,
        Err(_) => {
            // 如果文件不存在，我们仍然需要验证路径的安全性
            let normalized = normalize_path(&full_path);
            if !is_path_within_base(&normalized, base_path) {
                return Err(SecurityError::OutOfBounds);
            }
            return Ok(normalized);
        }
    };
    
    // 8. 确保规范化后的路径仍在基础目录内
    let canonical_base = base_path.canonicalize()
        .map_err(|_| SecurityError::InvalidBasePath)?;
    
    if !canonical_path.starts_with(canonical_base) {
        return Err(SecurityError::OutOfBounds);
    }
    
    Ok(canonical_path)
}

/// 手动规范化路径（用于文件不存在的情况）
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    
    for component in path.components() {
        match component {
            std::path::Component::Normal(name) => {
                components.push(name);
            }
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::CurDir => {
                // 忽略当前目录
            }
            _ => {
                // 保留其他组件（如根目录）
                components.clear();
                components.push(component.as_os_str());
            }
        }
    }
    
    components.iter().collect()
}

/// 检查路径是否在基础目录内
fn is_path_within_base(path: &Path, base: &Path) -> bool {
    let path_components: Vec<_> = path.components().collect();
    let base_components: Vec<_> = base.components().collect();
    
    if path_components.len() < base_components.len() {
        return false;
    }
    
    for (i, base_component) in base_components.iter().enumerate() {
        if path_components.get(i) != Some(base_component) {
            return false;
        }
    }
    
    true
}

/// 根据文件路径获取 Content-Type
pub fn get_content_type_for_file(file_path: &Path) -> String {
    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    match extension.as_str() {
        // Web 文件类型
        "html" | "htm" => "text/html".to_string(),
        "css" => "text/css".to_string(),
        "js" | "mjs" => "application/javascript".to_string(),
        "json" => "application/json".to_string(),
        "xml" => "application/xml".to_string(),
        
        // 图片文件类型
        "png" => "image/png".to_string(),
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "gif" => "image/gif".to_string(),
        "svg" => "image/svg+xml".to_string(),
        "webp" => "image/webp".to_string(),
        "ico" => "image/x-icon".to_string(),
        "bmp" => "image/bmp".to_string(),
        "tiff" | "tif" => "image/tiff".to_string(),
        
        // 字体文件类型
        "woff" => "font/woff".to_string(),
        "woff2" => "font/woff2".to_string(),
        "ttf" => "font/ttf".to_string(),
        "otf" => "font/otf".to_string(),
        "eot" => "application/vnd.ms-fontobject".to_string(),
        
        // 音频文件类型
        "mp3" => "audio/mpeg".to_string(),
        "wav" => "audio/wav".to_string(),
        "ogg" => "audio/ogg".to_string(),
        "m4a" => "audio/mp4".to_string(),
        "aac" => "audio/aac".to_string(),
        
        // 视频文件类型
        "mp4" => "video/mp4".to_string(),
        "webm" => "video/webm".to_string(),
        "avi" => "video/x-msvideo".to_string(),
        "mov" => "video/quicktime".to_string(),
        "wmv" => "video/x-ms-wmv".to_string(),
        
        // 文档文件类型
        "pdf" => "application/pdf".to_string(),
        "doc" => "application/msword".to_string(),
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
        "xls" => "application/vnd.ms-excel".to_string(),
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
        "ppt" => "application/vnd.ms-powerpoint".to_string(),
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation".to_string(),
        
        // 压缩文件类型
        "zip" => "application/zip".to_string(),
        "rar" => "application/vnd.rar".to_string(),
        "7z" => "application/x-7z-compressed".to_string(),
        "tar" => "application/x-tar".to_string(),
        "gz" => "application/gzip".to_string(),
        
        // 文本文件类型
        "txt" => "text/plain".to_string(),
        "md" => "text/markdown".to_string(),
        "csv" => "text/csv".to_string(),
        "log" => "text/plain".to_string(),
        
        // 其他常见类型
        "wasm" => "application/wasm".to_string(),
        "manifest" => "application/manifest+json".to_string(),
        "webmanifest" => "application/manifest+json".to_string(),
        
        // 默认类型
        _ => "application/octet-stream".to_string(),
    }
}

/// 使用字符集获取 Content-Type
pub fn get_content_type_with_charset(file_path: &Path) -> String {
    let content_type = get_content_type_for_file(file_path);
    
    // 为文本类型添加字符集
    if content_type.starts_with("text/") || 
       content_type == "application/javascript" ||
       content_type == "application/json" ||
       content_type == "application/xml" ||
       content_type == "image/svg+xml" {
        format!("{}; charset=utf-8", content_type)
    } else {
        content_type
    }
}

/// 检查文件类型是否可以被压缩
pub fn is_compressible_content_type(content_type: &str) -> bool {
    let compressible_types = [
        "text/",
        "application/javascript",
        "application/json",
        "application/xml",
        "image/svg+xml",
        "application/manifest+json",
    ];
    
    compressible_types.iter().any(|&prefix| content_type.starts_with(prefix))
}

/// 格式化 HTTP 日期 (RFC 2822 格式)
pub fn format_http_date_rfc2822(time: SystemTime) -> String {
    use chrono::{DateTime, Utc};
    
    let datetime: DateTime<Utc> = time.into();
    datetime.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}

/// 解析 HTTP 日期 (RFC 2822 格式)
pub fn parse_http_date_rfc2822(date_str: &str) -> Result<SystemTime, &'static str> {
    use chrono::{DateTime, Utc};
    
    // 尝试多种日期格式
    let formats = [
        "%a, %d %b %Y %H:%M:%S GMT",           // RFC 2822
        "%A, %d-%b-%y %H:%M:%S GMT",           // RFC 850
        "%a %b %d %H:%M:%S %Y",                // ANSI C asctime()
    ];
    
    for format in &formats {
        if let Ok(datetime) = DateTime::parse_from_str(date_str, format) {
            return Ok(datetime.with_timezone(&Utc).into());
        }
    }
    
    Err("Invalid date format")
}

/// 验证文件名是否安全
pub fn validate_filename(filename: &str) -> bool {
    // 检查文件名长度
    if filename.is_empty() || filename.len() > 255 {
        return false;
    }
    
    // 检查是否包含危险字符
    let dangerous_chars = ['<', '>', ':', '"', '|', '?', '*', '\0'];
    if filename.chars().any(|c| dangerous_chars.contains(&c)) {
        return false;
    }
    
    // 检查是否为保留名称（Windows）
    let reserved_names = [
        "CON", "PRN", "AUX", "NUL",
        "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
        "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"
    ];
    
    let name_upper = filename.to_uppercase();
    if reserved_names.contains(&name_upper.as_str()) {
        return false;
    }
    
    true
}

/// 计算文件统计信息
pub async fn calculate_directory_stats(config: &ResourceConfig) -> Result<FileStats, StaticFileError> {
    use tokio::fs;
    use std::collections::HashMap;
    
    let base_path = config.get_full_base_path();
    let mut stats = FileStats {
        total_files: 0,
        total_size: 0,
        by_type: HashMap::new(),
    };
    
    calculate_stats_recursive(&base_path, &mut stats).await?;
    
    Ok(stats)
}

/// 递归计算统计信息
fn calculate_stats_recursive<'a>(dir: &'a Path, stats: &'a mut FileStats) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), StaticFileError>> + 'a>> {
    Box::pin(async move {
        use tokio::fs;
        
        let mut entries = fs::read_dir(dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = entry.metadata().await?;
            
            if metadata.is_file() {
                let size = metadata.len();
                stats.total_files += 1;
                stats.total_size += size;
                
                // 按文件类型统计
                let content_type = get_content_type_for_file(&path);
                let type_stats = stats.by_type.entry(content_type).or_insert_with(TypeStats::new);
                type_stats.add_file(size);
                
            } else if metadata.is_dir() {
                // 递归处理子目录
                calculate_stats_recursive(&path, stats).await?;
            }
        }
        
        Ok(())
    })
}

/// 清理过期的缓存文件
pub async fn cleanup_expired_cache(config: &ResourceConfig, max_age_seconds: u64) -> Result<u64, StaticFileError> {
    use tokio::fs;
    use std::time::Duration;
    
    let base_path = config.get_full_base_path();
    let cutoff_time = SystemTime::now() - Duration::from_secs(max_age_seconds);
    let mut cleaned_count = 0;
    
    cleanup_recursive(&base_path, cutoff_time, &mut cleaned_count).await?;
    
    Ok(cleaned_count)
}

/// 递归清理过期文件
fn cleanup_recursive<'a>(dir: &'a Path, cutoff_time: SystemTime, count: &'a mut u64) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), StaticFileError>> + 'a>> {
    Box::pin(async move {
        use tokio::fs;
        
        let mut entries = fs::read_dir(dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = entry.metadata().await?;
            
            if metadata.is_file() {
                if let Ok(modified) = metadata.modified() {
                    if modified < cutoff_time {
                        if fs::remove_file(&path).await.is_ok() {
                            *count += 1;
                        }
                    }
                }
            } else if metadata.is_dir() {
                cleanup_recursive(&path, cutoff_time, count).await?;
            }
        }
        
        Ok(())
    })
}