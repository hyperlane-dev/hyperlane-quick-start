use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CicdService;

#[derive(Clone, Data, Debug, Default)]
pub(super) struct StepOutputBuilder {
    #[get(pub(crate))]
    pub(super) stdout: String,
    #[get(pub(crate))]
    pub(super) stderr: String,
    #[get(type(copy), pub(crate))]
    pub(super) is_timeout: bool,
    #[get(type(copy), pub(crate))]
    pub(super) timeout_secs: u64,
}

#[derive(Clone, Data, Debug, Serialize, Deserialize)]
pub struct LogEntry {
    #[get(type(copy), pub(crate))]
    pub(super) step_id: i32,
    #[get(pub(crate))]
    pub(super) content: String,
    #[get(type(copy), pub(crate))]
    pub(super) timestamp: i64,
    #[get(type(copy), pub(crate))]
    pub(super) is_stderr: bool,
}

#[derive(Clone, Data, Debug)]
pub struct StepStream {
    #[get(pub(crate))]
    pub(super) output: ArcRwLock<String>,
    #[get(pub(crate))]
    pub(super) status: ArcRwLock<CicdStatus>,
}

#[derive(Clone, Data, Debug)]
pub struct StepOutput {
    #[get(pub(crate))]
    pub(super) stdout: ArcRwLock<String>,
    #[get(pub(crate))]
    pub(super) stderr: ArcRwLock<String>,
}

#[derive(Clone, Data, Debug)]
pub struct LogStreamManager {
    #[get(pub(crate))]
    pub(super) broadcast_map: Arc<BroadcastMap<String>>,
    #[get(pub(crate))]
    pub(super) step_outputs: ArcRwLock<HashMap<i32, StepOutput>>,
    #[get(pub(crate))]
    pub(super) step_statuses: ArcRwLock<HashMap<i32, ArcRwLock<CicdStatus>>>,
    #[get(pub(crate))]
    pub(super) active_steps: ArcRwLock<HashMap<i32, HashSet<i32>>>,
}
