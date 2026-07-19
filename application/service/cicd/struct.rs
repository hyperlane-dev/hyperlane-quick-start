use super::*;

/// Service for managing CI/CD pipelines, runs, jobs, and step execution with Docker support.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CicdService;

/// Builder for accumulating step execution output including stdout, stderr, and timeout information.
#[derive(Clone, Data, Debug, Default)]
pub(super) struct StepOutputBuilder {
    /// The captured standard output stream content.
    pub(super) stdout: String,
    /// The captured standard error stream content.
    pub(super) stderr: String,
    /// Flag indicating whether the step execution timed out.
    #[get(type(copy))]
    pub(super) is_timeout: bool,
    /// The configured timeout duration in seconds.
    #[get(type(copy))]
    pub(super) timeout_secs: u64,
}

/// A single log entry representing a line of output from a CI/CD step.
#[derive(Clone, Data, Debug, Serialize, Deserialize)]
pub struct LogEntry {
    /// The foreign key referencing the step this log entry belongs to.
    #[get(type(copy))]
    pub(super) step_id: i32,
    /// The textual content of the log line.
    pub(super) content: String,
    /// The Unix timestamp (in milliseconds) when this log entry was produced.
    #[get(type(copy))]
    pub(super) timestamp: i64,
    /// Flag indicating whether this log entry originated from stderr.
    #[get(type(copy))]
    pub(super) is_stderr: bool,
}

/// A real-time streaming handle for a running CI/CD step, providing output and status updates.
#[derive(Clone, Data, Debug)]
pub struct StepStream {
    /// The shared output buffer for the step's combined stdout/stderr.
    pub(super) output: ArcRwLock<String>,
    /// The shared status tracker for the step's current execution state.
    pub(super) status: ArcRwLock<CicdStatus>,
}

/// A pair of read-write locked buffers for separating stdout and stderr of a step.
#[derive(Clone, Data, Debug)]
pub struct StepOutput {
    /// The shared buffer for standard output.
    pub(super) stdout: ArcRwLock<String>,
    /// The shared buffer for standard error.
    pub(super) stderr: ArcRwLock<String>,
}

/// Manager for broadcasting real-time log streams and tracking step outputs/statuses across active CI/CD runs.
#[derive(Clone, Data, Debug)]
pub struct LogStreamManager {
    /// The broadcast channel map for publishing log entries to subscribers.
    pub(super) broadcast_map: Arc<BroadcastMap<String>>,
    /// The map of step IDs to their separated output buffers.
    pub(super) step_outputs: ArcRwLock<HashMap<i32, StepOutput>>,
    /// The map of step IDs to their current execution status.
    pub(super) step_statuses: ArcRwLock<HashMap<i32, ArcRwLock<CicdStatus>>>,
    /// The map of run IDs to their sets of active step IDs.
    pub(super) active_steps: ArcRwLock<HashMap<i32, HashSet<i32>>>,
}
