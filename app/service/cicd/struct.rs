use super::*;

/// Result type for a stream reading task.
///
/// Inner `Result<String, String>` represents success (stdout/stderr content)
/// or failure (error message). Outer `Result` wraps the `JoinError` from
/// the async task join operation.
pub(super) type StreamResult = Result<Result<String, String>, JoinError>;

/// A pair of stream results for stdout and stderr.
pub(super) type StreamResultPair = (StreamResult, StreamResult);

/// Result type for timeout-wrapped stream operations.
///
/// Contains the `StreamResultPair` on success, or `Elapsed` error on timeout.
pub(super) type TimeoutResult = Result<StreamResultPair, Elapsed>;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CicdService;

pub(super) struct StepOutputBuilder {
    pub(super) stdout: String,
    pub(super) stderr: String,
    pub(super) is_timeout: bool,
    pub(super) timeout_secs: u64,
}

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub step_id: i32,
    pub content: String,
    pub timestamp: i64,
    pub is_stderr: bool,
}

#[derive(Clone, Debug)]
pub struct StepStream {
    pub sender: broadcast::Sender<LogEntry>,
    pub output: Arc<RwLock<String>>,
    pub status: Arc<RwLock<CicdStatus>>,
}

#[derive(Clone, Debug)]
pub struct LogStreamManager {
    pub streams: Arc<RwLock<HashMap<i32, HashMap<i32, StepStream>>>>,
}
