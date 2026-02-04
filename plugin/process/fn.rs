use super::*;

#[instrument_trace]
fn should_process_file(path: &Path) -> bool {
    let path_str: String = path.to_string_lossy().to_string();
    for excluded in EXCLUDED_DIRS.iter() {
        if path_str.contains(excluded) {
            return false;
        }
    }
    true
}

#[instrument_trace]
fn sort_derive_traits(content: &str) -> (String, bool) {
    let mut result: String = String::new();
    let mut changed: bool = false;
    let lines: Lines<'_> = content.lines();
    for line in lines {
        let trimmed: &str = line.trim();
        if trimmed.starts_with(HASH) && trimmed.contains("derive(") {
            let start_idx: usize = match trimmed.find("derive(") {
                Some(idx) => idx + 7,
                None => {
                    result.push_str(line);
                    result.push_str(BR);
                    continue;
                }
            };
            let end_idx: usize = match trimmed[start_idx..].find(')') {
                Some(idx) => start_idx + idx,
                None => {
                    result.push_str(line);
                    result.push_str(BR);
                    continue;
                }
            };
            let inner: &str = &trimmed[start_idx..end_idx];
            let traits: Vec<&str> = inner
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();
            let mut sorted_traits: Vec<&str> = traits.clone();
            sorted_traits.sort_by_key(|a| a.to_lowercase());
            if traits != sorted_traits {
                changed = true;
            }
            let sorted_inner: String = sorted_traits.join(", ");
            let prefix: &str = &trimmed[..start_idx];
            let suffix: &str = &trimmed[end_idx..];
            let new_line: String = format!("{prefix}{sorted_inner}{suffix}");
            let indent: &str = &line[..line.len() - line.trim_start().len()];
            result.push_str(indent);
            result.push_str(&new_line);
            result.push_str(BR);
        } else {
            result.push_str(line);
            result.push_str(BR);
        }
    }
    (result, changed)
}

#[instrument_trace]
fn process_derive_file(path: &Path) -> bool {
    let content: String = match read_to_string(path) {
        Ok(data) => data,
        Err(_) => return false,
    };
    let (new_content, changed): (String, bool) = sort_derive_traits(&content);
    if changed {
        let _ = write(path, new_content);
    }
    changed
}

#[instrument_trace]
fn fmt_derive_handler() {
    let root_dir: &Path = Path::new(POINT);
    let mut modified_count: usize = 0;
    fn visit_dir(dir: &Path, modified_count: &mut usize) {
        if let Ok(entries) = read_dir(dir) {
            for entry in entries.flatten() {
                let path: PathBuf = entry.path();
                if path.is_dir() {
                    if should_process_file(&path) {
                        visit_dir(&path, modified_count);
                    }
                } else if path.extension().is_some_and(|ext| ext == "rs")
                    && should_process_file(&path)
                    && process_derive_file(&path)
                {
                    info!("Modified{COLON_SPACE}{}", path.display());
                    *modified_count += 1;
                }
            }
        }
    }
    visit_dir(root_dir, &mut modified_count);
    info!("Total files modified{COLON_SPACE}{modified_count}");
}

#[instrument_trace]
fn fmt_handler() {
    info!("Running fmt-derive...");
    fmt_derive_handler();
    info!("Running cargo fmt...");
    let fmt_output: std::process::Output = Command::new("cargo")
        .args(["fmt"])
        .output()
        .expect("Failed to execute cargo fmt");
    if fmt_output.status.success() {
        info!("cargo fmt completed successfully");
    } else {
        error!(
            "cargo fmt failed{COLON_SPACE}{}",
            String::from_utf8_lossy(&fmt_output.stderr)
        );
    }
    info!("Running cargo clippy --fix...");
    let clippy_output: std::process::Output = Command::new("cargo")
        .args([
            "clippy",
            "--fix",
            "--workspace",
            "--all-targets",
            "--allow-dirty",
        ])
        .output()
        .expect("Failed to execute cargo clippy");
    if clippy_output.status.success() {
        info!("cargo clippy --fix completed successfully");
    } else {
        error!(
            "cargo clippy --fix failed{COLON_SPACE}{}",
            String::from_utf8_lossy(&clippy_output.stderr)
        );
    }
    info!("fmt command completed");
}

#[instrument_trace]
pub async fn create<P, F, Fut>(pid_path: P, server_hook: F)
where
    P: AsRef<str>,
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let args: Vec<String> = args().collect();
    debug!("Process create args{COLON_SPACE}{args:?}");
    let mut manager: ServerManager = ServerManager::new();
    manager
        .set_pid_file(pid_path.as_ref())
        .set_server_hook(server_hook);
    let is_daemon: bool = args.len() >= 3 && args[2].to_lowercase() == DAEMON_FLAG;
    let start_server = || async {
        if is_daemon {
            match manager.start_daemon().await {
                Ok(_) => info!("Server started in background successfully"),
                Err(error) => {
                    error!("Error starting server in background{COLON_SPACE}{error}")
                }
            };
        } else {
            info!("Server started successfully");
            manager.start().await;
        }
    };
    let stop_server = || async {
        match manager.stop().await {
            Ok(_) => info!("Server stopped successfully"),
            Err(error) => error!("Error stopping server{COLON_SPACE}{error}"),
        };
    };
    let hot_restart_server = || async {
        match manager
            .watch_detached(&["--clear", "--skip-local-deps", "-q", "-x", "run"])
            .await
        {
            Ok(_) => info!("Server started successfully"),
            Err(error) => error!("Error starting server in background{COLON_SPACE}{error}"),
        }
    };
    let restart_server = || async {
        stop_server().await;
        start_server().await;
    };
    if args.len() < 2 {
        warn!("No additional command-line parameters, default startup");
        start_server().await;
        return;
    }
    let command: String = args[1].to_lowercase();
    match command.as_str() {
        CMD_STOP => stop_server().await,
        CMD_RESTART => restart_server().await,
        CMD_HOT_RESTART => hot_restart_server().await,
        CMD_FMT => fmt_handler(),
        _ => {
            error!("Invalid command{COLON_SPACE}{command}");
        }
    }
}
