use super::*;

#[derive(Clone, Data, Debug, Default)]
pub struct DockerConfig {
    #[get(type(copy), pub)]
    pub(super) cpus: Option<f32>,
    #[get(type(copy), pub)]
    pub(super) disable_network: bool,
    pub(super) env_vars: Vec<(String, String)>,
    pub(super) image: String,
    pub(super) memory: Option<String>,
    #[get(type(copy), pub)]
    pub(super) pids_limit: Option<i32>,
    #[get(type(copy), pub)]
    pub(super) read_only: bool,
    pub(super) shell: String,
    pub(super) shell_flag: String,
    pub(super) volumes: Vec<(String, String)>,
    pub(super) workdir: String,
}

#[derive(Clone, Data, Debug, Default)]
pub struct DockerResult {
    #[get(type(copy), pub(crate))]
    pub(super) exit_code: i32,
    #[get(type(copy), pub(crate))]
    pub(super) success: bool,
    #[get(pub(crate))]
    pub(super) stderr: String,
    #[get(pub(crate))]
    pub(super) stdout: String,
}
