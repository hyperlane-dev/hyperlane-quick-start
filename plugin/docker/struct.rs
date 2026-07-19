use super::*;

/// Configuration for Docker container execution, defining resource limits and runtime settings.
#[derive(Clone, Data, Debug, Default)]
pub struct DockerConfig {
    /// The maximum number of CPUs allocated to the container.
    #[get(type(copy))]
    pub(super) cpus: Option<f32>,
    /// Whether network access is disabled for the container.
    #[get(type(copy))]
    pub(super) disable_network: bool,
    /// The environment variables passed to the container as key-value pairs.
    pub(super) env_vars: Vec<(String, String)>,
    /// The Docker image name used to create the container.
    pub(super) image: String,
    /// The memory limit applied to the container (e.g., "512m", "2g").
    pub(super) memory: Option<String>,
    /// The maximum number of processes allowed inside the container.
    #[get(type(copy))]
    pub(super) pids_limit: Option<i32>,
    /// Whether the container filesystem is mounted as read-only.
    #[get(type(copy))]
    pub(super) read_only: bool,
    /// The shell binary used to execute the command inside the container.
    pub(super) shell: String,
    /// The flag passed to the shell to execute a command string.
    pub(super) shell_flag: String,
    /// The volume mount mappings from host paths to container paths.
    pub(super) volumes: Vec<(String, String)>,
    /// The working directory inside the container where commands are executed.
    pub(super) workdir: String,
}

/// The result of a Docker command execution, capturing output and exit status.
#[derive(Clone, Data, Debug, Default)]
pub struct DockerResult {
    /// The exit code returned by the container process; -1 if unavailable.
    #[get(type(copy))]
    pub(super) exit_code: i32,
    /// Whether the container process exited successfully (exit code 0).
    #[get(type(copy))]
    pub(super) success: bool,
    /// The standard error output captured from the container process.
    pub(super) stderr: String,
    /// The standard output captured from the container process.
    pub(super) stdout: String,
}
