#[derive(Clone, Debug, Default)]
pub struct DockerConfig {
    pub(super) image: String,
    pub(super) shell: String,
    pub(super) shell_flag: String,
    pub(super) cpus: Option<f32>,
    pub(super) memory: Option<String>,
    pub(super) pids_limit: Option<i32>,
    pub(super) disable_network: bool,
    pub(super) read_only: bool,
    pub(super) workdir: String,
    pub(super) env_vars: Vec<(String, String)>,
    pub(super) volumes: Vec<(String, String)>,
}

#[derive(Clone, Debug, Default)]
pub struct DockerResult {
    pub(super) stdout: String,
    pub(super) stderr: String,
    pub(super) exit_code: i32,
    pub(super) success: bool,
}
