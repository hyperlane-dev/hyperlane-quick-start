use super::*;

#[derive(Clone, Data, Debug, Default)]
pub struct DockerConfig {
    #[get(type(copy), pub)]
    pub(super) cpus: Option<f32>,
    #[get(type(copy), pub)]
    pub(super) disable_network: bool,
    #[get(pub)]
    pub(super) env_vars: Vec<(String, String)>,
    #[get(pub)]
    pub(super) image: String,
    #[get(pub)]
    pub(super) memory: Option<String>,
    #[get(type(copy), pub)]
    pub(super) pids_limit: Option<i32>,
    #[get(type(copy), pub)]
    pub(super) read_only: bool,
    #[get(pub)]
    pub(super) shell: String,
    #[get(pub)]
    pub(super) shell_flag: String,
    #[get(pub)]
    pub(super) volumes: Vec<(String, String)>,
    #[get(pub)]
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
