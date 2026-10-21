/// The endpoint of the list of the projects.
pub(crate) const PATH_PROJECTS: &str = "/api/euv/playground/projects";

/// The endpoint that creates a project.
pub(crate) const PATH_PROJECTS_CREATE: &str = "/api/euv/playground/projects/create";

/// The endpoint of one project.
pub(crate) const PATH_PROJECTS_GET: &str = "/api/euv/playground/projects/get";

/// The endpoint that writes a project.
pub(crate) const PATH_PROJECTS_SAVE: &str = "/api/euv/playground/projects/save";

/// The endpoint that removes a project.
pub(crate) const PATH_PROJECTS_DELETE: &str = "/api/euv/playground/projects/delete";

/// The endpoint that starts a build.
pub(crate) const PATH_RUN: &str = "/api/euv/playground/run";

/// The endpoint of the status of a build.
pub(crate) const PATH_RUN_STATUS: &str = "/api/euv/playground/run/status";

/// The endpoint of the template of a new project.
pub(crate) const PATH_DEFAULT_CODE: &str = "/api/euv/playground/default-code";

/// The status of a build that waits for the worker.
pub(crate) const STATUS_PENDING: &str = "pending";

/// The status of a build that is being compiled.
pub(crate) const STATUS_RUNNING: &str = "running";

/// The status of a build that succeeded.
pub(crate) const STATUS_SUCCESS: &str = "success";

/// The status of a build that failed.
pub(crate) const STATUS_FAILED: &str = "failed";

/// The number of milliseconds the page waits between two polls of a build.
pub(crate) const POLL_INTERVAL_MS: i32 = 1500;

/// The number of milliseconds the page waits before it writes the code.
pub(crate) const SAVE_DELAY_MS: i32 = 600;

/// The number of milliseconds the page waits at most for a build.
pub(crate) const POLL_TIMEOUT_MS: i64 = 600_000;

/// The number of milliseconds in a second.
pub(crate) const MILLIS_SECOND: i64 = 1000;

/// The language the editor of the playground shows.
pub(crate) const LANGUAGE_RUST: &str = "rust";

/// The text an empty value starts with.
pub(crate) const TEXT_EMPTY: &str = "";

/// The failure reported when the name of a new project is empty.
pub(crate) const MESSAGE_NAME_REQUIRED: &str = "A project needs a name";

/// The state of the page after the code was written into the server.
pub(crate) const MESSAGE_SAVED: &str = "Saved";

/// The state of the page after the code could not be written.
pub(crate) const MESSAGE_SAVE_FAILED: &str = "Save failed";

/// The failure reported when the visitor starts a build without a project.
pub(crate) const MESSAGE_NO_PROJECT: &str = "Select or create a project first";

/// The state of the page while the build is being started.
pub(crate) const MESSAGE_SUBMITTING: &str = "Submitting";

/// The state of the page after the server refused the build.
pub(crate) const MESSAGE_SUBMIT_FAILED: &str = "Submit failed";

/// The state of the page after the build took longer than the timeout.
pub(crate) const MESSAGE_TIMEOUT: &str = "Build timed out";

/// The failure reported after the build took longer than the timeout.
pub(crate) const MESSAGE_TIMEOUT_DETAIL: &str =
    "The build did not finish in time, check the logs of the server";

/// The state of the page while the build waits for the worker.
pub(crate) const MESSAGE_QUEUED: &str = "Queued";

/// The state of the page while the build is being compiled.
pub(crate) const MESSAGE_BUILDING: &str = "Building";

/// The state of the page after the build succeeded.
pub(crate) const MESSAGE_RUNNING: &str = "Running";

/// The state of the page after the build failed.
pub(crate) const MESSAGE_BUILD_FAILED: &str = "Build failed";

/// The state of the page after the server reported a status the page does
/// not know.
pub(crate) const MESSAGE_UNKNOWN_STATUS: &str = "Unknown status";

/// The failure reported when the server does not describe the failure.
pub(crate) const MESSAGE_NO_DETAIL: &str = "The server did not describe the failure";
