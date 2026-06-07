use super::*;

/// Executes a command inside a default Docker container.
///
/// Uses the default `DockerConfig` with security restrictions applied.
///
/// # Arguments
/// - `&str`: The shell command to execute inside the container.
///
/// # Returns
/// - `DockerResult`: The result of the command execution including stdout, stderr, and exit code.
#[instrument_trace]
pub async fn execute(command: &str) -> DockerResult {
    let config: DockerConfig = DockerConfig::new();
    execute_with_config(command, &config).await
}

/// Executes a command inside a Docker container with the specified configuration.
///
/// Returns an error result if the command string is empty or if Docker execution fails.
///
/// # Arguments
/// - `&str`: The shell command to execute inside the container.
/// - `&DockerConfig`: The Docker configuration controlling resource limits and container settings.
///
/// # Returns
/// - `DockerResult`: The result of the command execution including stdout, stderr, and exit code.
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

/// Builds the complete list of Docker CLI arguments from the given configuration and command.
///
/// Constructs arguments for `docker run` including resource limits, volume mounts,
/// environment variables, and the command to execute.
///
/// # Arguments
/// - `&DockerConfig`: The Docker configuration to convert into CLI arguments.
/// - `&str`: The shell command to append as the final argument.
///
/// # Returns
/// - `Vec<String>`: The ordered list of arguments for the `docker` command.
#[instrument_trace]
fn build_docker_args(config: &DockerConfig, command: &str) -> Vec<String> {
    let mut args: Vec<String> = vec!["run".to_string(), "--rm".to_string()];
    if config.get_disable_network() {
        args.push("--network=none".to_string());
    }
    if let Some(cpus) = config.get_cpus() {
        args.push(format!("--cpus={cpus}"));
    }
    if let Some(memory) = config.try_get_memory() {
        args.push(format!("--memory={memory}"));
    }
    if let Some(pids_limit) = config.get_pids_limit() {
        args.push(format!("--pids-limit={pids_limit}"));
    }
    if config.get_read_only() {
        args.push("--read-only".to_string());
        args.push("--tmpfs".to_string());
        args.push("/tmp:rw,noexec,nosuid,size=100m".to_string());
    }
    for (host_path, container_path) in config.get_volumes() {
        args.push("-v".to_string());
        args.push(format!("{host_path}:{container_path}"));
    }
    for (key, value) in config.get_env_vars() {
        args.push("-e".to_string());
        args.push(format!("{key}={value}"));
    }
    args.push("-w".to_string());
    args.push(config.get_workdir().clone());
    args.push(config.get_image().clone());
    args.push(config.get_shell().clone());
    args.push(config.get_shell_flag().clone());
    args.push(command.to_string());
    args
}

/// Checks whether Docker is available on the host system by running `docker --version`.
///
/// # Returns
/// - `bool`: `true` if Docker is installed and accessible, `false` otherwise.
#[instrument_trace]
pub async fn is_docker_available() -> bool {
    match Command::new("docker").arg("--version").output().await {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
