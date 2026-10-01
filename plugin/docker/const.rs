/// Name of the Docker CLI executable.
pub const DOCKER_COMMAND_BIN: &str = "docker";

/// Docker CLI flag printing the client version.
pub const DOCKER_VERSION_FLAG: &str = "--version";

/// Error message returned when the command string is empty.
pub const DOCKER_NO_COMMAND_ERROR: &str = "No command to execute";

/// Default container image used for command execution.
pub const DOCKER_DEFAULT_IMAGE: &str = "alpine:latest";

/// Default working directory inside the container.
pub const DOCKER_DEFAULT_WORKDIR: &str = "/workspace";

/// Default memory limit applied to a container.
pub const DOCKER_DEFAULT_MEMORY_LIMIT: &str = "512m";

/// Formatted result for a command that succeeded without output.
pub const DOCKER_SUCCESS_NO_OUTPUT: &str = "Command executed successfully (no output)";

/// Docker run flag removing the container after it exits.
pub const DOCKER_FLAG_AUTO_REMOVE: &str = "--rm";

/// Docker run flag disabling container networking.
pub const DOCKER_FLAG_NETWORK_NONE: &str = "--network=none";

/// Docker run flag mounting the root filesystem read-only.
pub const DOCKER_FLAG_READ_ONLY: &str = "--read-only";

/// Docker run flag mounting an in-memory temporary filesystem.
pub const DOCKER_FLAG_TMPFS: &str = "--tmpfs";

/// Mount specification for the read-only container tmpfs.
pub const DOCKER_TMPFS_MOUNT_SPEC: &str = "/tmp:rw,noexec,nosuid,size=100m";
