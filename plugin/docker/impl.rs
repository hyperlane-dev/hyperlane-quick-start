use super::*;

impl DockerConfig {
    #[instrument_trace]
    pub fn secure() -> Self {
        Self {
            cpus: Some(1.0),
            memory: Some("512m".to_string()),
            disable_network: true,
            ..Self::new()
        }
    }

    #[instrument_trace]
    pub fn with_network() -> Self {
        Self {
            disable_network: false,
            ..Self::new()
        }
    }

    #[instrument_trace]
    pub fn high_resource() -> Self {
        Self {
            cpus: Some(4.0),
            memory: Some("2g".to_string()),
            ..Self::new()
        }
    }
}

impl DockerConfig {
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
            env_vars: Vec::new(),
            volumes: Vec::new(),
        }
    }

    #[instrument_trace]
    pub fn image(mut self, image: &str) -> Self {
        self.set_image(image.to_string());
        self
    }

    #[instrument_trace]
    pub fn cpus(mut self, cpus: f32) -> Self {
        self.set_cpus(Some(cpus));
        self
    }

    #[instrument_trace]
    pub fn memory(mut self, memory: &str) -> Self {
        self.set_memory(Some(memory.to_string()));
        self
    }

    #[instrument_trace]
    pub fn network(mut self, enabled: bool) -> Self {
        self.set_disable_network(!enabled);
        self
    }

    #[instrument_trace]
    pub fn volume(mut self, host_path: &str, container_path: &str) -> Self {
        let mut volumes: Vec<(String, String)> = self.get_volumes().to_vec();
        volumes.push((host_path.to_string(), container_path.to_string()));
        self.set_volumes(volumes);
        self
    }

    #[instrument_trace]
    pub fn env(mut self, key: &str, value: &str) -> Self {
        let mut env_vars: Vec<(String, String)> = self.get_env_vars().to_vec();
        env_vars.push((key.to_string(), value.to_string()));
        self.set_env_vars(env_vars);
        self
    }
}

impl DockerResult {
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
                "Error: Exit code {}\nStdout: {}\nStderr: {}",
                self.get_exit_code(),
                self.get_stdout().trim(),
                self.get_stderr().trim()
            )
        }
    }
}
