use super::*;

fn get_date_directories(log_dir_path: &Path) -> Vec<String> {
    let mut date_dirs: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(log_dir_path) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                if let Some(dir_name) = entry.file_name().to_str() {
                    date_dirs.push(dir_name.to_string());
                }
            }
        }
    }

    date_dirs.sort();
    date_dirs.reverse();
    date_dirs
}

fn get_log_files(date_path: &Path) -> Vec<String> {
    let mut log_files: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(date_path) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".log") {
                        log_files.push(file_name.to_string());
                    }
                }
            }
        }
    }

    log_files.sort();
    log_files.reverse();
    log_files
}

async fn read_and_process_log_file(full_path: &str, date_dir: &str, log_file: &str) -> String {
    match async_read_from_file::<Vec<u8>>(full_path).await {
        Ok(content) => {
            let content_str: String = String::from_utf8_lossy(&content).to_string();
            if !content_str.trim().is_empty() {
                let mut lines: Vec<&str> = content_str.lines().collect();
                lines.reverse();
                lines.join("\n")
            } else {
                String::new()
            }
        }
        Err(_) => format!("Failed to read file: {}/{}", date_dir, log_file),
    }
}

async fn process_date_directory(log_dir: &str, date_dir: &str) -> Vec<String> {
    let date_path: String = format!("{}/{}", log_dir, date_dir);
    let date_path_obj: &Path = Path::new(&date_path);
    let log_files: Vec<String> = get_log_files(date_path_obj);
    let mut logs: Vec<String> = Vec::new();

    for log_file in log_files.iter().take(5) {
        let full_path: String = format!("{}/{}", date_path, log_file);
        let content: String = read_and_process_log_file(&full_path, date_dir, log_file).await;
        if !content.is_empty() {
            logs.push(content);
        }
    }

    logs
}

pub async fn read_log_file(level: &str) -> String {
    let log_dir: String = format!("{}/{}", SERVER_LOG_DIR, level);
    let log_dir_path: &Path = Path::new(&log_dir);

    if !log_dir_path.exists() {
        return format!("Log directory not found: {}", log_dir);
    }

    let date_dirs: Vec<String> = get_date_directories(log_dir_path);
    let mut all_logs: Vec<String> = Vec::new();

    for date_dir in date_dirs.iter().take(7) {
        let logs: Vec<String> = process_date_directory(&log_dir, date_dir).await;
        all_logs.extend(logs);
    }

    if all_logs.is_empty() {
        format!("No {} logs found in {}", level, log_dir)
    } else {
        all_logs.join("\n\n")
    }
}
