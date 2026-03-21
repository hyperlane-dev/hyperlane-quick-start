use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CicdService;

#[derive(Clone, Data, Debug, Default)]
pub(super) struct StepOutputBuilder {
    pub(super) stdout: String,
    pub(super) stderr: String,
    #[get(type(copy))]
    pub(super) is_timeout: bool,
    #[get(type(copy))]
    pub(super) timeout_secs: u64,
}

#[derive(Clone, Data, Debug, Serialize, Deserialize)]
pub struct LogEntry {
    #[get(type(copy))]
    pub(super) step_id: i32,
    pub(super) content: String,
    #[get(type(copy))]
    pub(super) timestamp: i64,
    #[get(type(copy))]
    pub(super) is_stderr: bool,
}

#[derive(Clone, Data, Debug)]
pub struct StepStream {
    pub(super) output: ArcRwLock<String>,
    pub(super) status: ArcRwLock<CicdStatus>,
}

#[derive(Clone, Data, Debug)]
pub struct StepOutput {
    pub(super) stdout: ArcRwLock<String>,
    pub(super) stderr: ArcRwLock<String>,
}

#[derive(Clone, Data, Debug)]
pub struct LogStreamManager {
    pub(super) broadcast_map: Arc<BroadcastMap<String>>,
    pub(super) step_outputs: ArcRwLock<HashMap<i32, StepOutput>>,
    pub(super) step_statuses: ArcRwLock<HashMap<i32, ArcRwLock<CicdStatus>>>,
    pub(super) active_steps: ArcRwLock<HashMap<i32, HashSet<i32>>>,
}
