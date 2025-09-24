use super::*;
use std::time::SystemTime;



/// 检查是否应该返回 304 Not Modified
pub fn should_return_not_modified(
    response: &StaticFileResponse,
    if_modified_since: &Option<String>,
    if_none_match: &Option<String>,
) -> bool {
    // 检查 If-None-Match (ETag)
    if let (Some(etag), Some(if_none_match_value)) = (&response.etag, if_none_match) {
        if etag == if_none_match_value || if_none_match_value == "*" {
            return true;
        }
    }
    
    // 检查 If-Modified-Since
    if let (Some(last_modified), Some(if_modified_since_value)) = (&response.last_modified, if_modified_since) {
        if let Ok(client_time) = parse_http_date_rfc2822(if_modified_since_value) {
            if *last_modified <= client_time {
                return true;
            }
        }
    }
    
    false
}

/// 解析 HTTP Range 头
pub fn parse_range_header(range_header: &str, file_size: u64) -> Result<Vec<HttpRange>, &'static str> {
    if !range_header.starts_with("bytes=") {
        return Err("Invalid range header format");
    }
    
    let ranges_str = &range_header[6..]; // 移除 "bytes=" 前缀
    let mut ranges = Vec::new();
    
    for range_spec in ranges_str.split(',') {
        let range_spec = range_spec.trim();
        
        if let Some(dash_pos) = range_spec.find('-') {
            let (start_str, end_str) = range_spec.split_at(dash_pos);
            let end_str = &end_str[1..]; // 移除 '-'
            
            let start = if start_str.is_empty() {
                // 后缀范围: -500 (最后500字节)
                if let Ok(suffix_length) = end_str.parse::<u64>() {
                    if suffix_length >= file_size {
                        0
                    } else {
                        (file_size - suffix_length) as usize
                    }
                } else {
                    return Err("Invalid suffix range");
                }
            } else if let Ok(start_val) = start_str.parse::<u64>() {
                if start_val >= file_size {
                    return Err("Range start exceeds file size");
                }
                start_val as usize
            } else {
                return Err("Invalid range start");
            };
            
            let end = if end_str.is_empty() {
                // 前缀范围: 500- (从500字节到文件末尾)
                (file_size - 1) as usize
            } else if let Ok(end_val) = end_str.parse::<u64>() {
                if end_val >= file_size {
                    (file_size - 1) as usize
                } else {
                    end_val as usize
                }
            } else {
                return Err("Invalid range end");
            };
            
            if start <= end {
                ranges.push(HttpRange { start, end });
            } else {
                return Err("Invalid range: start > end");
            }
        } else {
            return Err("Invalid range format");
        }
    }
    
    if ranges.is_empty() {
        return Err("No valid ranges found");
    }
    
    Ok(ranges)
}

/// 获取缓存控制头
pub fn get_cache_control_header(content_type: &str) -> String {
    // 根据内容类型确定缓存策略
    if content_type.starts_with("text/html") {
        "no-cache, must-revalidate".to_string()
    } else if content_type.starts_with("application/json") {
        "no-cache, must-revalidate".to_string()
    } else if content_type.starts_with("text/css") 
        || content_type.starts_with("application/javascript") 
        || content_type.starts_with("image/") {
        "public, max-age=31536000, immutable".to_string() // 1年缓存
    } else {
        "public, max-age=3600".to_string() // 1小时缓存
    }
}

/// 获取 Expires 头
pub fn get_expires_header(content_type: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH, Duration};
    
    let cache_seconds = if content_type.starts_with("text/html") 
        || content_type.starts_with("application/json") {
        0 // 不缓存
    } else if content_type.starts_with("text/css") 
        || content_type.starts_with("application/javascript") 
        || content_type.starts_with("image/") {
        31536000 // 1年
    } else {
        3600 // 1小时
    };
    
    if cache_seconds == 0 {
        // 过去的时间，表示立即过期
        "Thu, 01 Jan 1970 00:00:00 GMT".to_string()
    } else {
        let expires_time = SystemTime::now() + Duration::from_secs(cache_seconds);
        format_http_date_rfc2822(expires_time)
    }
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