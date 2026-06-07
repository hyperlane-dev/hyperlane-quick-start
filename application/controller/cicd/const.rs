/// SSE initial poll delay ms.
pub const SSE_INITIAL_POLL_DELAY_MS: u64 = 500;

/// SSE connection timeout secs.
pub const SSE_CONNECTION_TIMEOUT_SECS: u64 = 3600;

/// SSE idle sleep ms.
pub const SSE_IDLE_SLEEP_MS: u64 = 10;

/// Directory path for cicd view redirect path.
pub const CICD_VIEW_REDIRECT_PATH: &str = "/static/cicd/index.html";

/// SSE event log.
pub const SSE_EVENT_LOG: &str = "log";

/// SSE event complete.
pub const SSE_EVENT_COMPLETE: &str = "complete";

/// SSE reason no active streams.
pub const SSE_REASON_NO_ACTIVE_STREAMS: &str = "no_active_streams";

/// SSE reason timeout.
pub const SSE_REASON_TIMEOUT: &str = "timeout";

/// SSE reason run completed.
pub const SSE_REASON_RUN_COMPLETED: &str = "run_completed";

/// Error message when missing or invalid id.
pub const ERROR_MISSING_OR_INVALID_ID: &str = "Missing or invalid id parameter";

/// Error message when missing or invalid run id.
pub const ERROR_MISSING_OR_INVALID_RUN_ID: &str = "Missing or invalid run_id parameter";

/// Error message when pipeline not found.
pub const ERROR_PIPELINE_NOT_FOUND: &str = "Pipeline not found";

/// Error message when run not found.
pub const ERROR_RUN_NOT_FOUND: &str = "Run not found";

/// Status code for success job updated.
pub const SUCCESS_JOB_STATUS_UPDATED: &str = "Job status updated successfully";

/// Status code for success step updated.
pub const SUCCESS_STEP_STATUS_UPDATED: &str = "Step status updated successfully";
