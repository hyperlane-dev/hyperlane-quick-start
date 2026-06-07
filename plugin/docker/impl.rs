use super::*;

/// Preset configuration constructors for `DockerConfig`.
impl DockerConfig {
    /// Creates a secure Docker configuration with restricted resources and no network access.
    ///
    /// # Returns
    ///
    /// - `DockerConfig`: A configuration with 1 CPU, 512MB memory, and network disabled.
    #[instrument_trace]
    pub fn secure() -> Self {
        Self {
            cpus: Some(1.0),
            memory: Some("512m".to_string()),
            disable_network: true,
            ..Self::new()
        }
    }

    /// Creates a Docker configuration with network access enabled.
    ///
    /// # Returns
    ///
    /// - `DockerConfig`: A configuration that allows container network connectivity.
    #[instrument_trace]
    pub fn with_network() -> Self {
        Self {
            disable_network: false,
            ..Self::new()
        }
    }

    /// Creates a Docker configuration with high resource limits.
    ///
    /// # Returns
    ///
    /// - `DockerConfig`: A configuration with 4 CPUs and 2GB memory allocation.
    #[instrument_trace]
    pub fn high_resource() -> Self {
        Self {
            cpus: Some(4.0),
            memory: Some("2g".to_string()),
            ..Self::new()
        }
    }
}

/// Builder-style configuration methods for `DockerConfig`.
impl DockerConfig {
    /// Creates a new `DockerConfig` with default values.
    ///
    /// Defaults to alpine:latest image, sh shell, 1 CPU, 512MB memory,
    /// 100 PID limit, network disabled, read-only filesystem, and /workspace working directory.
    ///
    /// # Returns
    ///
    /// - `DockerConfig`: A new configuration with sensible security defaults.
    #[instrument_trace]
    pub fn new() -> Self {
        Self {
            image: "alpine:latest".to_string(),
            shell: "sh".to_string(),
            shell_flag: "-c".to_string(),
            cpus: Some(1.0),
            memory: Some("512m".to_string()),
            pids_limit: Some(100),
            disable_network: true,
            read_only: true,
            workdir: "/workspace".to_string(),
            env_vars: vec![],
            volumes: vec![],
        }
    }

    /// Sets the Docker image to use for container execution.
    ///
    /// # Arguments
    ///
    /// - `&str`: The Docker image name (e.g., "alpine:latest", "ubuntu:22.04").
    ///
    /// # Returns
    ///
    /// - `DockerConfig`: The updated configuration for method chaining.
    #[instrument_trace]
    pub fn image(mut self, image: &str) -> Self {
        self.set_image(image.to_string());
        self
    }

    /// Sets the CPU limit for the container.
    ///
    /// # Arguments
    ///
    /// - `f32`: The number of CPUs to allocate (e.g., 1.0, 2.5).
    ///
    /// # Returns
    ///
    /// - `DockerConfig`: The updated configuration for method chaining.
    #[instrument_trace]
    pub fn cpus(mut self, cpus: f32) -> Self {
        self.set_cpus(Some(cpus));
        self
    }

    /// Sets the memory limit for the container.
    ///
    /// # Arguments
    ///
    /// - `&str`: The memory limit string (e.g., "512m", "2g").
    ///
    /// # Returns
    ///
    /// - `DockerConfig`: The updated configuration for method chaining.
    #[instrument_trace]
    pub fn memory(mut self, memory: &str) -> Self {
        self.set_memory(Some(memory.to_string()));
        self
    }

    /// Enables or disables network access for the container.
    ///
    /// # Arguments
    ///
    /// - `bool`: Whether network access should be enabled.
    ///
    /// # Returns
    ///
    /// - `DockerConfig`: The updated configuration for method chaining.
    #[instrument_trace]
    pub fn network(mut self, enabled: bool) -> Self {
        self.set_disable_network(!enabled);
        self
    }

    /// Adds a volume mount mapping from the host to the container.
    ///
    /// # Arguments
    ///
    /// - `&str`: The absolute path on the host machine.
    /// - `&str`: The absolute path inside the container.
    ///
    /// # Returns
    ///
    /// - `DockerConfig`: The updated configuration for method chaining.
    #[instrument_trace]
    pub fn volume(mut self, host_path: &str, container_path: &str) -> Self {
        let mut volumes: Vec<(String, String)> = self.get_volumes().to_vec();
        volumes.push((host_path.to_string(), container_path.to_string()));
        self.set_volumes(volumes);
        self
    }

    /// Adds an environment variable to the container.
    ///
    /// # Arguments
    ///
    /// - `&str`: The environment variable name.
    /// - `&str`: The environment variable value.
    ///
    /// # Returns
    ///
    /// - `DockerConfig`: The updated configuration for method chaining.
    #[instrument_trace]
    pub fn env(mut self, key: &str, value: &str) -> Self {
        let mut env_vars: Vec<(String, String)> = self.get_env_vars().to_vec();
        env_vars.push((key.to_string(), value.to_string()));
        self.set_env_vars(env_vars);
        self
    }
}

/// Conversion and formatting methods for `DockerResult`.
impl DockerResult {
    /// Constructs a `DockerResult` from a process `Output` by extracting stdout, stderr, and exit status.
    ///
    /// # Arguments
    ///
    /// - `Output`: The raw output from an asynchronous command execution.
    ///
    /// # Returns
    ///
    /// - `DockerResult`: A result containing the decoded stdout, stderr, exit code, and success flag.
    ///
    /// # Panics
    ///
    /// Does not panic; uses `unwrap_or(-1)` as fallback for missing exit code.
    #[instrument_trace]
    pub fn from_output(output: Output) -> Self {
        let stdout: String = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr: String = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code: i32 = output.status.code().unwrap_or(-1);
        let success: bool = output.status.success();
        Self {
            stdout,
            stderr,
            exit_code,
            success,
        }
    }

    /// Formats the execution result into a human-readable string.
    ///
    /// Returns the trimmed stdout on success, or a detailed error message
    /// including exit code, stdout, and stderr on failure.
    ///
    /// # Returns
    ///
    /// - `String`: A formatted representation of the command execution result.
    #[instrument_trace]
    pub fn format_output(&self) -> String {
        if self.get_success() {
            if self.get_stdout().is_empty() && self.get_stderr().is_empty() {
                "Command executed successfully (no output)".to_string()
            } else if self.get_stdout().is_empty() {
                format!("Stderr: {}", self.get_stderr().trim())
            } else {
                self.get_stdout().trim().to_string()
            }
        } else {
            format!(
                "Error: Exit code {}{BR}Stdout: {}{BR}Stderr: {}",
                self.get_exit_code(),
                self.get_stdout().trim(),
                self.get_stderr().trim()
            )
        }
    }
}
