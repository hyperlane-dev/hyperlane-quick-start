use super::*;

/// Timeout value for task timeout.
pub const TASK_TIMEOUT: Duration = Duration::from_secs(1800);

/// Error message when pipeline config content required.
pub const ERROR_PIPELINE_CONFIG_CONTENT_REQUIRED: &str = "Pipeline config content is required";

/// Command for no to execute.
pub const NO_COMMAND_TO_EXECUTE: &str = "No command to execute";

/// Identifier string for the local (in-process) CI/CD job runner.
pub const LOCAL_RUNNER: &str = "local-runner";

/// Default shell windows.
pub const DEFAULT_SHELL_WINDOWS: &str = "cmd.exe";

/// Default shell unix.
pub const DEFAULT_SHELL_UNIX: &str = "sh";

/// Error message when failed to take stdout.
pub const ERROR_FAILED_TO_TAKE_STDOUT: &str = "Failed to take stdout";

/// Error message when failed to take stderr.
pub const ERROR_FAILED_TO_TAKE_STDERR: &str = "Failed to take stderr";

/// Error message when failed to read stdout.
pub const ERROR_FAILED_TO_READ_STDOUT: &str = "Failed to read stdout";

/// Error message when failed to read stderr.
pub const ERROR_FAILED_TO_READ_STDERR: &str = "Failed to read stderr";

/// Output stream read buffer size.
pub const OUTPUT_STREAM_BUFFER_SIZE: usize = 8192;

/// Error message when prefix.
pub const ERROR_PREFIX: &str = "Error:";

/// Output label for stdout.
pub const OUTPUT_LABEL_STDOUT: &str = "[Stdout]";

/// Output label for stderr.
pub const OUTPUT_LABEL_STDERR: &str = "[Stderr]";

/// Output label for timeout.
pub const OUTPUT_LABEL_TIMEOUT: &str = "[Timeout]";

/// No output message.
pub const NO_OUTPUT_MESSAGE: &str = "Command executed successfully (no output)";

/// Server restart interrupt message.
pub const SERVER_RESTART_INTERRUPT_MESSAGE: &str =
    "[System] Task was interrupted due to server restart";

/// Default page size.
pub const DEFAULT_PAGE_SIZE: u64 = 50;

/// Maximum page size.
pub const MAX_PAGE_SIZE: i32 = 100;

/// SSE timeout seconds.
pub const SSE_TIMEOUT_SECONDS: u64 = 3600;

/// SSE initial delay millis.
pub const SSE_INITIAL_DELAY_MILLIS: u64 = 500;

/// SSE poll interval millis.
pub const SSE_POLL_INTERVAL_MILLIS: u64 = 10;
