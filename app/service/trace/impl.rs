use super::*;

impl TraceService {
    pub async fn search_trace(trace: &str) -> String {
        let base_dir: &Path = Path::new(SERVER_LOG_DIR);
        if !base_dir.exists() {
            return format!("Log directory not found: {}", base_dir.display());
        }
        let mut prev_line: Option<String> = None;
        for level in SERVER_LOG_LEVEL {
            let log_dir: PathBuf = base_dir.join(level);
            if !log_dir.exists() {
                continue;
            }
            let date_dirs: Vec<String> = LogService::get_sorted_dirs(&log_dir);
            for date_dir in date_dirs.iter().take(MAX_DATE_DIRS) {
                let date_path: PathBuf = log_dir.join(date_dir);
                let log_files: Vec<String> = LogService::get_sorted_log_files(&date_path);
                for log_file_name in log_files.iter().take(MAX_LOG_FILES_PER_DATE) {
                    let full_path: PathBuf = date_path.join(log_file_name);
                    if let Ok(content) =
                        async_read_from_file::<Vec<u8>>(full_path.to_str().unwrap_or_default())
                            .await
                    {
                        let content_str: String = String::from_utf8_lossy(&content).to_string();
                        for line in content_str.lines() {
                            if line.trim().contains(&format!("\"{TRACE}\": [\"{trace}\"]")) {
                                return if let Some(prev) = &prev_line {
                                    format!("{prev}{BR}{BR}{line}{BR}")
                                } else {
                                    format!("{line}{BR}")
                                };
                            }
                            prev_line = Some(line.to_string());
                        }
                    }
                }
            }
        }
        format!("No trace found with value: {trace}")
    }
}
