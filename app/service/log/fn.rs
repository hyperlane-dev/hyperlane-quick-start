use super::*;

fn get_sorted_dirs(path: &Path) -> Vec<String> {
    fs::read_dir(path)
        .map(|entries| {
            let mut dirs: Vec<String> = entries
                .filter_map(Result::ok)
                .filter(|e| e.file_type().map_or(false, |ft| ft.is_dir()))
                .filter_map(|e| e.file_name().into_string().ok())
                .collect();
            dirs.sort();
            dirs.reverse();
            dirs
        })
        .unwrap_or_default()
}

fn get_sorted_log_files(path: &Path) -> Vec<String> {
    fs::read_dir(path)
        .map(|entries| {
            let mut files: Vec<String> = entries
                .filter_map(Result::ok)
                .filter(|e| e.file_type().map_or(false, |ft| ft.is_file()))
                .filter_map(|e| e.file_name().into_string().ok())
                .filter(|name| name.ends_with(LOG_FILE_EXTENSION))
                .collect();
            files.sort();
            files.reverse();
            files
        })
        .unwrap_or_default()
}

async fn read_and_reverse_log_file(full_path: &Path) -> Result<String, String> {
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

async fn process_date_directory(log_dir: &Path, date_dir: &str) -> Vec<String> {
    let date_path: PathBuf = log_dir.join(date_dir);
    let log_files: Vec<String> = get_sorted_log_files(&date_path);
    let mut logs: Vec<String> = Vec::new();
    for log_file_name in log_files.iter().take(MAX_LOG_FILES_PER_DATE) {
        let full_path: PathBuf = date_path.join(log_file_name);
        if let Ok(content) = read_and_reverse_log_file(&full_path).await {
            if !content.is_empty() {
                logs.push(content);
            }
        }
    }
    logs
}

pub async fn read_log_file(level: &str) -> String {
    let log_dir: PathBuf = Path::new(SERVER_LOG_DIR).join(level);
    if !log_dir.exists() {
        return format!("Log directory not found: {}", log_dir.display());
    }
    let date_dirs: Vec<String> = get_sorted_dirs(&log_dir);
    let mut all_logs: Vec<String> = Vec::new();
    for date_dir in date_dirs.iter().take(MAX_DATE_DIRS) {
        let logs: Vec<String> = process_date_directory(&log_dir, date_dir).await;
        all_logs.extend(logs);
    }
    if all_logs.is_empty() {
        format!("No {} logs found in {}", level, log_dir.display())
    } else {
        all_logs.join(BR)
    }
}

pub async fn search_trace(trace: &str) -> String {
    let log_dir: &Path = Path::new(SERVER_LOG_DIR);
    if !log_dir.exists() {
        return format!("Log directory not found: {}", log_dir.display());
    }
    let re: Regex =
        Regex::new(r"(?s)\$(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})\$ \$$([A-Za-z]+)\$ => (.*?);")
            .unwrap();
    let mut results: Vec<String> = Vec::new();
    let mut dir_queue: VecDeque<PathBuf> = VecDeque::new();
    dir_queue.push_back(log_dir.to_path_buf());
    while let Some(current_path) = dir_queue.pop_front() {
        if let Ok(entries) = fs::read_dir(&current_path) {
            for entry in entries.filter_map(Result::ok) {
                let path: PathBuf = entry.path();
                if path.is_dir() {
                    dir_queue.push_back(path);
                } else if path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map_or(false, |ext| {
                        ext == LOG_FILE_EXTENSION.trim_start_matches('.')
                    })
                {
                    if let Ok(content) = read_and_reverse_log_file(&path).await {
                        for entry in content.lines().collect::<Vec<&str>>() {
                            if let Some(caps) = re.captures(entry) {
                                let timestamp_str: &str =
                                    caps.get(1).map_or("", |m| m.as_str().trim_end());
                                let entry_type: &str =
                                    caps.get(2).map_or("", |m| m.as_str().trim_end());
                                let content: &str =
                                    caps.get(3).map_or("", |m| m.as_str().trim_end());
                                if entry_type == "Request" || entry_type == "Response" {
                                    if let Ok(trace_re) = Regex::new(trace) {
                                        if trace_re.is_match(content) {
                                            results.push(format!(
                                                "Found trace in {}:\nTime: {}\nType: {}\nContent: {}",
                                                path.display(),
                                                timestamp_str,
                                                entry_type,
                                                content
                                            ));
                                        }
                                    } else if content.contains(trace) {
                                        results.push(format!(
                                            "Found trace in {}:\nTime: {}\nType: {}\nContent: {}",
                                            path.display(),
                                            timestamp_str,
                                            entry_type,
                                            content
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if results.is_empty() {
        format!("Trace {} not found", trace)
    } else {
        results.join("\n\n")
    }
}
