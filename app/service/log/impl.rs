use super::*;

impl LogService {
    #[instrument_trace]
    pub fn get_sorted_dirs(path: &Path) -> Vec<String> {
        fs::read_dir(path)
            .map(|entries| {
                let mut dirs: Vec<String> = entries
                    .filter_map(Result::ok)
                    .filter(|error| error.file_type().is_ok_and(|ft| ft.is_dir()))
                    .filter_map(|error| error.file_name().into_string().ok())
                    .collect();
                dirs.sort();
                dirs.reverse();
                dirs
            })
            .unwrap_or_default()
    }

    #[instrument_trace]
    pub fn get_sorted_log_files(path: &Path) -> Vec<String> {
        fs::read_dir(path)
            .map(|entries| {
                let mut files: Vec<String> = entries
                    .filter_map(Result::ok)
                    .filter(|error| error.file_type().is_ok_and(|ft| ft.is_file()))
                    .filter_map(|error| error.file_name().into_string().ok())
                    .filter(|name| name.ends_with(LOG_FILE_EXTENSION))
                    .collect();
                files.sort();
                files.reverse();
                files
            })
            .unwrap_or_default()
    }

    #[instrument_trace]
    pub async fn read_and_reverse_log_file(full_path: &Path) -> Result<String, String> {
        async_read_from_file::<Vec<u8>>(full_path.to_str().unwrap_or_default())
            .await
            .map(|content| {
                let content_str: String = String::from_utf8_lossy(&content).to_string();
                if content_str.trim().is_empty() {
                    String::new()
                } else {
                    content_str.lines().rev().collect::<Vec<&str>>().join(BR)
                }
            })
            .map_err(|_| {
                format!(
                    "Failed to read file: {}",
                    full_path.to_str().unwrap_or("invalid path")
                )
            })
    }

    #[instrument_trace]
    pub async fn process_date_directory(log_dir: &Path, date_dir: &str) -> Vec<String> {
        let date_path: PathBuf = log_dir.join(date_dir);
        let log_files: Vec<String> = Self::get_sorted_log_files(&date_path);
        let mut logs: Vec<String> = Vec::new();
        for log_file_name in log_files.iter().take(MAX_LOG_FILES_PER_DATE) {
            let full_path: PathBuf = date_path.join(log_file_name);
            if let Ok(content) = Self::read_and_reverse_log_file(&full_path).await {
                if !content.is_empty() {
                    logs.push(content);
                }
            }
        }
        logs
    }

    #[instrument_trace]
    pub async fn read_log_file(level: Level) -> String {
        let log_dir: PathBuf = Path::new(SERVER_LOG_DIR).join(level.to_string());
        if !log_dir.exists() {
            return format!("Log directory not found: {}", log_dir.display());
        }
        let level_string: String = level.to_string();
        let log_dir_string: String = log_dir.display().to_string();
        let date_dirs: Vec<String> = Self::get_sorted_dirs(&log_dir);
        let mut all_logs: Vec<String> = Vec::new();
        for date_dir in date_dirs.iter().take(MAX_DATE_DIRS) {
            let logs: Vec<String> = Self::process_date_directory(&log_dir, date_dir).await;
            all_logs.extend(logs);
        }
        if all_logs.is_empty() {
            format!("No {level_string} logs found in {log_dir_string}")
        } else {
            all_logs.join(BR)
        }
    }
}
