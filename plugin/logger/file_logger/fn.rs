use super::*;

/// Extracts the second element (index number) from log filenames in a directory.
///
/// # Arguments
///
/// - `&str` - The directory path to scan for log files.
///
/// # Returns
///
/// - `usize` - The extracted index number or default start index.
pub(crate) fn get_second_element_from_filename(dir_path: &str) -> usize {
    let mut res_idx: usize = DEFAULT_LOG_FILE_START_IDX;
    if let Ok(entries) = read_dir(dir_path) {
        for entry in entries.filter_map(Result::ok) {
            let file_name: String = entry.file_name().to_string_lossy().to_string();
            let parts: Vec<&str> = file_name.split(POINT).collect();
            if parts.len() > 1
                && let Ok(second_element) = parts[1].parse::<usize>()
            {
                res_idx = second_element.max(res_idx);
            }
        }
    }
    res_idx.max(DEFAULT_LOG_FILE_START_IDX)
}

/// Generates a log filename with given index using current date.
///
/// # Arguments
///
/// - `usize` - The index number for the log file.
///
/// # Returns
///
/// - `String` - The formatted log filename with path.
#[inline(always)]
pub(crate) fn get_file_name(idx: usize) -> String {
    format!(
        "{}{}{}{}{}{}",
        ROOT_PATH,
        date(),
        POINT,
        idx,
        POINT,
        LOG_EXTENSION
    )
}

/// Generates directory name for current date's logs.
///
/// # Returns
///
/// - `String` - The directory name based on current date.
#[inline(always)]
pub(crate) fn get_file_dir_name() -> String {
    format!("{}{}", ROOT_PATH, date())
}

/// Constructs appropriate log file path considering size limits.
///
/// # Arguments
///
/// - `&str` - The system directory path.
/// - `&str` - The base path for logs.
/// - `&usize` - The maximum allowed file size in bytes.
///
/// # Returns
///
/// - `String` - The full path to appropriate log file.
pub(crate) fn get_log_path(system_dir: &str, base_path: &str, limit_file_size: &usize) -> String {
    let mut combined_path: String = base_path.trim_end_matches(ROOT_PATH).to_string();
    if !system_dir.starts_with(ROOT_PATH) {
        combined_path.push_str(ROOT_PATH);
    }
    combined_path.push_str(
        system_dir
            .trim_start_matches(ROOT_PATH)
            .trim_end_matches(ROOT_PATH),
    );
    combined_path.push_str(&get_file_dir_name());
    let idx: usize = get_second_element_from_filename(&combined_path);
    let mut combined_path_clone: String = combined_path.clone();
    combined_path.push_str(&get_file_name(idx));
    let file_size: usize = get_file_size(&combined_path).unwrap_or_default() as usize;
    if &file_size <= limit_file_size {
        return combined_path;
    }
    combined_path_clone.push_str(&get_file_name(idx + 1));
    combined_path_clone
}

/// Formats log data with timestamp for each line.
///
/// # Arguments
///
/// - `T` - The data to be logged.
///
/// # Returns
///
/// - `String` - The formatted log string with timestamps.
#[inline(always)]
pub fn common_log<T: AsRef<str>>(data: T) -> String {
    let mut log_string: String = String::new();
    for line in data.as_ref().lines() {
        let line_string: String = format!("{} {}{}", time(), line, BR);
        log_string.push_str(&line_string);
    }
    log_string
}

/// Handles log data formatting by delegating to common_log.
///
/// # Arguments
///
/// - `T` - The data to be logged.
///
/// # Returns
///
/// - `String` - The formatted log string.
#[inline(always)]
pub fn log_handler<T: AsRef<str>>(log_data: T) -> String {
    common_log(log_data)
}
