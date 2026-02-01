use super::*;

impl DockerConfig {
    #[instrument_trace]
    pub fn secure() -> Self {
        Self::new().cpus(1.0).memory("512m").network(false)
    }

    #[instrument_trace]
    pub fn with_network() -> Self {
        let mut config: Self = Self::new();
        config.disable_network = false;
        config
    }

    #[instrument_trace]
    pub fn high_resource() -> Self {
        Self::new().cpus(4.0).memory("2g")
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
        self.image = image.to_string();
        self
    }

    #[instrument_trace]
    pub fn cpus(mut self, cpus: f32) -> Self {
        self.cpus = Some(cpus);
        self
    }

    #[instrument_trace]
    pub fn memory(mut self, memory: &str) -> Self {
        self.memory = Some(memory.to_string());
        self
    }

    #[instrument_trace]
    pub fn network(mut self, enabled: bool) -> Self {
        self.disable_network = !enabled;
        self
    }

    #[instrument_trace]
    pub fn volume(mut self, host_path: &str, container_path: &str) -> Self {
        self.volumes
            .push((host_path.to_string(), container_path.to_string()));
        self
    }

    #[instrument_trace]
    pub fn env(mut self, key: &str, value: &str) -> Self {
        self.env_vars.push((key.to_string(), value.to_string()));
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
        if self.success {
            if self.stdout.is_empty() && self.stderr.is_empty() {
                "Command executed successfully (no output)".to_string()
            } else if self.stdout.is_empty() {
                format!("Stderr: {}", self.stderr.trim())
            } else {
                self.stdout.trim().to_string()
            }
        } else {
            format!(
                "Error: Exit code {}\nStdout: {}\nStderr: {}",
                self.exit_code,
                self.stdout.trim(),
                self.stderr.trim()
            )
        }
    }
}
