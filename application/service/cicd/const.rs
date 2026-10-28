use super::*;

/// Output label for timeout.
pub const OUTPUT_LABEL_TIMEOUT: &str = "[Timeout]";

/// Server restart interrupt message.
pub const SERVER_RESTART_INTERRUPT_MESSAGE: &str =
    "[System] Task was interrupted due to server restart";

/// SSE timeout seconds.
pub const SSE_TIMEOUT_SECONDS: u64 = 3600;

/// SSE initial delay millis.
pub const SSE_INITIAL_DELAY_MILLIS: u64 = 500;

/// SSE poll interval millis.
pub const SSE_POLL_INTERVAL_MILLIS: u64 = 10;

/// Timeout value for task timeout.
pub(crate) const TASK_TIMEOUT: Duration = Duration::from_secs(1800);

/// Error message when pipeline config content required.
pub(crate) const ERROR_PIPELINE_CONFIG_CONTENT_REQUIRED: &str =
    "Pipeline config content is required";

/// Command for no to execute.
pub(crate) const NO_COMMAND_TO_EXECUTE: &str = "No command to execute";

/// Identifier string for the local (in-process) CI/CD job runner.
pub(crate) const LOCAL_RUNNER: &str = "local-runner";

/// Default shell windows.
pub(crate) const DEFAULT_SHELL_WINDOWS: &str = "cmd.exe";

/// Default shell unix.
pub(crate) const DEFAULT_SHELL_UNIX: &str = "sh";

/// Error message when failed to take stdout.
pub(crate) const ERROR_FAILED_TO_TAKE_STDOUT: &str = "Failed to take stdout";

/// Error message when failed to take stderr.
pub(crate) const ERROR_FAILED_TO_TAKE_STDERR: &str = "Failed to take stderr";

/// Error message when failed to read stdout.
pub(crate) const ERROR_FAILED_TO_READ_STDOUT: &str = "Failed to read stdout";

/// Error message when failed to read stderr.
pub(crate) const ERROR_FAILED_TO_READ_STDERR: &str = "Failed to read stderr";

/// Output stream read buffer size.
pub(crate) const OUTPUT_STREAM_BUFFER_SIZE: usize = 8192;

/// Error message when prefix.
pub(crate) const ERROR_PREFIX: &str = "Error:";

/// Output label for stdout.
pub(crate) const OUTPUT_LABEL_STDOUT: &str = "[Stdout]";

/// Output label for stderr.
pub(crate) const OUTPUT_LABEL_STDERR: &str = "[Stderr]";

/// No output message.
pub(crate) const NO_OUTPUT_MESSAGE: &str = "Command executed successfully (no output)";

/// Default page size.
pub(crate) const DEFAULT_PAGE_SIZE: u64 = 50;

/// Maximum page size.
pub(crate) const MAX_PAGE_SIZE: i32 = 100;

/// Windows environment variable naming the command processor.
pub(crate) const ENV_COMSPEC: &str = "COMSPEC";

/// Environment variable naming the login shell.
pub(crate) const ENV_SHELL: &str = "SHELL";

/// Log line emitted when a task is interrupted by a server restart.
pub(crate) const LOG_TASK_INTERRUPTED: &str = "[System] Task was interrupted due to server restart";
