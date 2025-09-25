use super::*;
use std::path::Path;
use std::time::SystemTime;
use tokio::fs;

/// 统一的静态资源文件服务
pub async fn serve_unified_resource_file(
    config: &ResourceConfig,
    path: &str,
) -> Result<StaticFileResponse, StaticFileError> {
    // 1. 验证路径安全性
    let safe_path = validate_unified_path_security(config, path)?;

    // 2. 检查文件是否存在且为文件（不是目录）
    let metadata = fs::metadata(&safe_path).await?;

    if metadata.is_dir() {
        return Err(StaticFileError::Forbidden);
    }

    // 3. 检查文件大小
    let file_size = metadata.len();
    if file_size > config.max_file_size {
        return Err(StaticFileError::FileTooLarge);
    }

    // 4. 读取文件内容
    let content = fs::read(&safe_path).await?;

    // 5. 获取文件的最后修改时间
    let last_modified = metadata.modified().ok();

    // 6. 生成 ETag
    let etag = generate_unified_etag(&metadata, &safe_path);

    // 7. 确定 Content-Type
    let content_type = get_content_type_with_charset(&safe_path);

    Ok(StaticFileResponse::new(
        content,
        content_type,
        last_modified,
        etag,
        file_size,
    ))
}

/// 原有的静态资源文件服务（保持向后兼容）
pub async fn serve_resource_file(path: &str) -> Result<StaticFileResponse, StaticFileError> {
    let config = ResourceConfig::static_resources();
    serve_unified_resource_file(&config, path).await
}

/// 生成统一的 ETag
fn generate_unified_etag(metadata: &std::fs::Metadata, path: &Path) -> Option<String> {
    // 使用文件大小和修改时间生成 ETag
    if let Ok(modified) = metadata.modified() {
        if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
            let etag = format!(
                "\"{}-{}-{}\"",
                duration.as_secs(),
                metadata.len(),
                simple_hash(&path.to_string_lossy())
            );
            return Some(etag);
        }
    }

    // 如果无法获取修改时间，使用文件大小和路径哈希
    let path_str = path.to_string_lossy();
    let hash = simple_hash(&path_str);
    Some(format!("\"{}-{}\"", hash, metadata.len()))
}

/// 简单哈希函数
fn simple_hash(s: &str) -> u64 {
    let mut hash = 0u64;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    hash
}

/// 检查文件是否存在
pub async fn file_exists_unified(config: &ResourceConfig, path: &str) -> bool {
    if let Ok(safe_path) = validate_unified_path_security(config, path) {
        if let Ok(metadata) = fs::metadata(&safe_path).await {
            return metadata.is_file();
        }
    }
    false
}

/// 获取文件信息（不读取内容）
pub async fn get_file_info_unified(
    config: &ResourceConfig,
    path: &str,
) -> Result<(u64, SystemTime, String), StaticFileError> {
    let safe_path = validate_unified_path_security(config, path)?;
    let metadata = fs::metadata(&safe_path).await?;

    if metadata.is_dir() {
        return Err(StaticFileError::Forbidden);
    }

    let file_size = metadata.len();
    let last_modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let etag = generate_unified_etag(&metadata, &safe_path).unwrap_or_default();

    Ok((file_size, last_modified, etag))
}

/// 批量检查文件存在性
pub async fn batch_check_files_exist(
    config: &ResourceConfig,
    paths: &[String],
) -> Vec<(String, bool)> {
    let mut results = Vec::new();

    for path in paths {
        let exists = file_exists_unified(config, path).await;
        results.push((path.clone(), exists));
    }

    results
}

/// 获取目录下的文件列表（如果允许）
pub async fn list_directory_files(
    config: &ResourceConfig,
    path: &str,
) -> Result<Vec<String>, StaticFileError> {
    if !config.allow_directory_listing {
        return Err(StaticFileError::Forbidden);
    }

    let safe_path = validate_unified_path_security(config, path)?;

    if !safe_path.is_dir() {
        return Err(StaticFileError::NotFound);
    }

    let mut entries = fs::read_dir(&safe_path).await?;
    let mut files = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        if let Ok(metadata) = entry.metadata().await {
            if metadata.is_file() {
                if let Some(filename) = entry.file_name().to_str() {
                    files.push(filename.to_string());
                }
            }
        }
    }

    files.sort();
    Ok(files)
}

/// 获取文件的详细信息
pub async fn get_detailed_file_info(
    config: &ResourceConfig,
    path: &str,
) -> Result<DetailedFileInfo, StaticFileError> {
    let safe_path = validate_unified_path_security(config, path)?;
    let metadata = fs::metadata(&safe_path).await?;

    if metadata.is_dir() {
        return Err(StaticFileError::Forbidden);
    }

    let file_size = metadata.len();
    let last_modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    let created = metadata.created().unwrap_or(SystemTime::UNIX_EPOCH);
    let etag = generate_unified_etag(&metadata, &safe_path).unwrap_or_default();
    let content_type = get_content_type_with_charset(&safe_path);
    let is_compressible = is_compressible_content_type(&content_type);

    Ok(DetailedFileInfo {
        path: path.to_string(),
        file_size,
        last_modified,
        created,
        etag,
        content_type: content_type.clone(),
        is_compressible,
        is_cacheable: !content_type.starts_with("text/html"),
    })
}

/// 流式读取大文件
pub async fn stream_large_file(
    config: &ResourceConfig,
    path: &str,
    range: Option<(u64, u64)>,
) -> Result<StreamFileResponse, StaticFileError> {
    let safe_path = validate_unified_path_security(config, path)?;
    let metadata = fs::metadata(&safe_path).await?;

    if metadata.is_dir() {
        return Err(StaticFileError::Forbidden);
    }

    let file_size = metadata.len();
    if file_size > config.max_file_size {
        return Err(StaticFileError::FileTooLarge);
    }

    let (start, end) = match range {
        Some((s, e)) => {
            let end = if e >= file_size { file_size - 1 } else { e };
            if s > end {
                return Err(StaticFileError::InvalidPath);
            }
            (s, end)
        }
        None => (0, file_size - 1),
    };

    let content_length = end - start + 1;
    let content_type = get_content_type_with_charset(&safe_path);
    let last_modified = metadata.modified().ok();
    let etag = generate_unified_etag(&metadata, &safe_path);

    Ok(StreamFileResponse {
        file_path: safe_path,
        start,
        end,
        content_length,
        total_size: file_size,
        content_type,
        last_modified,
        etag,
    })
}
