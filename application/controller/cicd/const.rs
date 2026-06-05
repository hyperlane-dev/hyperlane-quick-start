pub const SSE_INITIAL_POLL_DELAY_MS: u64 = 500;

pub const SSE_CONNECTION_TIMEOUT_SECS: u64 = 3600;

pub const SSE_IDLE_SLEEP_MS: u64 = 10;

pub const CICD_VIEW_REDIRECT_PATH: &str = "/static/cicd/index.html";

pub const SSE_EVENT_LOG: &str = "log";

pub const SSE_EVENT_COMPLETE: &str = "complete";

pub const SSE_REASON_NO_ACTIVE_STREAMS: &str = "no_active_streams";

pub const SSE_REASON_TIMEOUT: &str = "timeout";

pub const SSE_REASON_RUN_COMPLETED: &str = "run_completed";

pub const ERROR_MISSING_OR_INVALID_ID: &str = "Missing or invalid id parameter";

pub const ERROR_MISSING_OR_INVALID_RUN_ID: &str = "Missing or invalid run_id parameter";

pub const ERROR_PIPELINE_NOT_FOUND: &str = "Pipeline not found";

pub const ERROR_RUN_NOT_FOUND: &str = "Run not found";

pub const SUCCESS_JOB_STATUS_UPDATED: &str = "Job status updated successfully";

pub const SUCCESS_STEP_STATUS_UPDATED: &str = "Step status updated successfully";
