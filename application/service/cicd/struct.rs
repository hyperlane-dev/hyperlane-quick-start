use super::*;

/// Service for managing CI/CD pipelines, runs, jobs, and step execution with Docker support.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CicdService;

/// Builder for accumulating step execution output including stdout, stderr, and timeout information.
#[derive(Clone, Data, Debug, Default)]
pub(super) struct StepOutputBuilder {
    /// The captured standard output stream content.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) stdout: String,
    /// The captured standard error stream content.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) stderr: String,
    /// Flag indicating whether the step execution timed out.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_timeout: bool,
    /// The configured timeout duration in seconds.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) timeout_secs: u64,
}

/// A single log entry representing a line of output from a CI/CD step.
#[derive(Clone, Data, Debug, Serialize, Deserialize)]
pub struct LogEntry {
    /// The foreign key referencing the step this log entry belongs to.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) step_id: i32,
    /// The textual content of the log line.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content: String,
    /// The Unix timestamp (in milliseconds) when this log entry was produced.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) timestamp: i64,
    /// Flag indicating whether this log entry originated from stderr.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_stderr: bool,
}

/// A real-time streaming handle for a running CI/CD step, providing output and status updates.
#[derive(Clone, Data, Debug)]
pub struct StepStream {
    /// The shared output buffer for the step's combined stdout/stderr.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub output: ArcRwLock<String>,
    /// The shared status tracker for the step's current execution state.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub status: ArcRwLock<CicdStatus>,
}

/// A pair of read-write locked buffers for separating stdout and stderr of a step.
#[derive(Clone, Data, Debug)]
pub struct StepOutput {
    /// The shared buffer for standard output.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) stdout: ArcRwLock<String>,
    /// The shared buffer for standard error.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) stderr: ArcRwLock<String>,
}

/// Manager for broadcasting real-time log streams and tracking step outputs/statuses across active CI/CD runs.
#[derive(Clone, Data, Debug)]
pub struct LogStreamManager {
    /// The broadcast channel map for publishing log entries to subscribers.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) broadcast_map: Arc<BroadcastMap<String>>,
    /// The map of step IDs to their separated output buffers.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) step_outputs: ArcRwLock<HashMap<i32, StepOutput>>,
    /// The map of step IDs to their current execution status.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) step_statuses: ArcRwLock<HashMap<i32, ArcRwLock<CicdStatus>>>,
    /// The map of run IDs to their sets of active step IDs.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) active_steps: ArcRwLock<HashMap<i32, HashSet<i32>>>,
}
