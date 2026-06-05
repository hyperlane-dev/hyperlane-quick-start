use super::*;

pub const TASK_TIMEOUT: Duration = Duration::from_secs(1800);
pub const ERROR_PIPELINE_CONFIG_CONTENT_REQUIRED: &str = "Pipeline config content is required";
pub const NO_COMMAND_TO_EXECUTE: &str = "No command to execute";
pub const LOCAL_RUNNER: &str = "local-runner";
pub const DEFAULT_SHELL_WINDOWS: &str = "cmd.exe";
pub const DEFAULT_SHELL_UNIX: &str = "bash";
pub const ERROR_FAILED_TO_TAKE_STDOUT: &str = "Failed to take stdout";
pub const ERROR_FAILED_TO_TAKE_STDERR: &str = "Failed to take stderr";
pub const ERROR_PREFIX: &str = "Error:";
pub const OUTPUT_LABEL_STDOUT: &str = "[Stdout]";
pub const OUTPUT_LABEL_STDERR: &str = "[Stderr]";
pub const OUTPUT_LABEL_TIMEOUT: &str = "[Timeout]";
pub const NO_OUTPUT_MESSAGE: &str = "Command executed successfully (no output)";
pub const SERVER_RESTART_INTERRUPT_MESSAGE: &str =
    "[System] Task was interrupted due to server restart";
pub const DEFAULT_PAGE_SIZE: u64 = 50;
pub const MAX_PAGE_SIZE: i32 = 100;
pub const SSE_TIMEOUT_SECONDS: u64 = 3600;
pub const SSE_INITIAL_DELAY_MILLIS: u64 = 500;
pub const SSE_POLL_INTERVAL_MILLIS: u64 = 10;
