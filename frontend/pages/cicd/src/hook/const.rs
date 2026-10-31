/// The endpoint that lists the pipelines of the server.
pub(crate) const PATH_PIPELINE_LIST: &str = "/api/cicd/pipeline/list";

/// The endpoint that creates a pipeline.
pub(crate) const PATH_PIPELINE_CREATE: &str = "/api/cicd/pipeline/create";

/// The endpoint that triggers a run of a pipeline.
pub(crate) const PATH_RUN_TRIGGER: &str = "/api/cicd/run/trigger";

/// The endpoint that lists the runs of the server.
pub(crate) const PATH_RUN_LIST: &str = "/api/cicd/run/list";

/// The endpoint that answers with the detail of one run.
pub(crate) const PATH_RUN_DETAIL: &str = "/api/cicd/run/detail";

/// The endpoint that streams the log of one run.
pub(crate) const PATH_RUN_LOGS: &str = "/api/cicd/run/logs/stream";

/// The number of runs one page holds.
pub(crate) const RUNS_PAGE_SIZE: i32 = 50;

/// The tab that lists the pipelines.
pub(crate) const TAB_PIPELINES: &str = "pipelines";

/// The tab that lists the runs.
pub(crate) const TAB_RUNS: &str = "runs";

/// The name of the visitor that triggers the runs.
pub(crate) const TRIGGERED_BY: &str = "web";

/// The event of the log stream that carries one line.
pub(crate) const EVENT_LOG: &str = "log";

/// The event of the log stream that closes the stream.
pub(crate) const EVENT_COMPLETE: &str = "complete";

/// The empty text of the page.
pub(crate) const TEXT_EMPTY: &str = "";

/// The number of milliseconds in a second.
pub(crate) const MILLIS_PER_SECOND: i64 = 1_000;
