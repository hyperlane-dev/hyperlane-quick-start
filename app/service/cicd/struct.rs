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
    pub(super) output: Arc<RwLock<String>>,
    #[get(pub(crate))]
    pub(super) status: Arc<RwLock<CicdStatus>>,
}

#[derive(Clone, Data, Debug)]
pub struct LogStreamManager {
    #[get(pub(crate))]
    pub(super) broadcast_map: Arc<BroadcastMap<String>>,
    #[get(pub(crate))]
    pub(super) step_outputs: Arc<RwLock<HashMap<i32, Arc<RwLock<String>>>>>,
    #[get(pub(crate))]
    pub(super) step_statuses: Arc<RwLock<HashMap<i32, Arc<RwLock<CicdStatus>>>>>,
    #[get(pub(crate))]
    pub(super) active_steps: Arc<RwLock<HashMap<i32, HashSet<i32>>>>,
}
