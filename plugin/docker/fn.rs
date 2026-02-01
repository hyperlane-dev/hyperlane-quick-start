use super::*;

#[instrument_trace]
pub async fn execute(command: &str) -> DockerResult {
    let config: DockerConfig = DockerConfig::new();
    execute_with_config(command, &config).await
}

#[instrument_trace]
pub async fn execute_with_config(command: &str, config: &DockerConfig) -> DockerResult {
    if command.is_empty() {
        return DockerResult {
            stdout: String::new(),
            stderr: "No command to execute".to_string(),
            exit_code: -1,
            success: false,
        };
    }
    let args: Vec<String> = build_docker_args(config, command);
    let output_result: Result<Output, std::io::Error> =
        Command::new("docker").args(&args).output().await;
    match output_result {
        Ok(output) => DockerResult::from_output(output),
        Err(error) => DockerResult {
            stdout: String::new(),
            stderr: format!("Failed to execute docker command: {error}"),
            exit_code: -1,
            success: false,
        },
    }
}

#[instrument_trace]
fn build_docker_args(config: &DockerConfig, command: &str) -> Vec<String> {
    let mut args: Vec<String> = vec!["run".to_string(), "--rm".to_string()];
    if config.disable_network {
        args.push("--network=none".to_string());
    }
    if let Some(cpus) = config.cpus {
        args.push(format!("--cpus={cpus}"));
    }
    if let Some(ref memory) = config.memory {
        args.push(format!("--memory={memory}"));
    }
    if let Some(pids_limit) = config.pids_limit {
        args.push(format!("--pids-limit={pids_limit}"));
    }
    if config.read_only {
        args.push("--read-only".to_string());
        args.push("--tmpfs".to_string());
        args.push("/tmp:rw,noexec,nosuid,size=100m".to_string());
    }
    for (host_path, container_path) in &config.volumes {
        args.push("-v".to_string());
        args.push(format!("{host_path}:{container_path}"));
    }
    for (key, value) in &config.env_vars {
        args.push("-e".to_string());
        args.push(format!("{key}={value}"));
    }
    args.push("-w".to_string());
    args.push(config.workdir.clone());
    args.push(config.image.clone());
    args.push(config.shell.clone());
    args.push(config.shell_flag.clone());
    args.push(command.to_string());
    args
}

#[instrument_trace]
pub async fn is_docker_available() -> bool {
    match Command::new("docker").arg("--version").output().await {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
