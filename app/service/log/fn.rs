use super::*;
use std::fs;
use std::path::Path;

pub async fn read_log_file(level: &str) -> String {
    let log_dir: String = format!("{}/{}", SERVER_LOG_DIR, level);
    let log_dir_path: &Path = Path::new(&log_dir);

    if !log_dir_path.exists() {
        return format!("Log directory not found: {}", log_dir);
    }

    let mut all_logs: Vec<String> = Vec::new();
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

    for date_dir in date_dirs.iter().take(7) {
        let date_path: String = format!("{}/{}", log_dir, date_dir);
        let date_path_obj: &Path = Path::new(&date_path);

        if let Ok(entries) = fs::read_dir(date_path_obj) {
            let mut log_files: Vec<String> = Vec::new();

            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name.ends_with(".log") {
                            log_files.push(file_name.to_string());
                        }
                    }
                }
            }

            log_files.sort();
            log_files.reverse();

            for log_file in log_files.iter().take(5) {
                let full_path: String = format!("{}/{}", date_path, log_file);
                match async_read_from_file::<Vec<u8>>(&full_path).await {
                    Ok(content) => {
                        let content_str: String = String::from_utf8_lossy(&content).to_string();
                        if !content_str.trim().is_empty() {
                            let mut lines: Vec<&str> = content_str.lines().collect();
                            lines.reverse();
                            let reversed_content: String = lines.join("\n");
                            all_logs.push(reversed_content);
                        }
                    }
                    Err(_) => {
                        all_logs.push(format!("Failed to read file: {}/{}", date_dir, log_file));
                    }
                }
            }
        }
    }

    if all_logs.is_empty() {
        format!("No {} logs found in {}", level, log_dir)
    } else {
        all_logs.join("\n\n")
    }
}
