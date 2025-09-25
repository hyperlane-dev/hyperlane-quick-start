use super::*;

/// 解析 HTTP Range 头
pub fn parse_range_header(
    range_header: &str,
    file_size: u64,
) -> Result<Vec<HttpRange>, &'static str> {
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
