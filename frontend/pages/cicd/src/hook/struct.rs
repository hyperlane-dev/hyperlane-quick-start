use super::*;

/// The reactive state the CI/CD page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct CicdState {
    /// The tab being displayed.
    #[get(type(copy))]
    pub tab: Signal<String>,
    /// The pipelines of the server.
    #[get(type(copy))]
    pub pipelines: Signal<Vec<Pipeline>>,
    /// The runs of the server.
    #[get(type(copy))]
    pub runs: Signal<Vec<Run>>,
    /// The number of the runs the server holds in total.
    #[get(type(copy))]
    pub total: Signal<i32>,
    /// Whether the server holds older runs.
    #[get(type(copy))]
    pub has_more: Signal<bool>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
    /// The message of the last success.
    #[get(type(copy))]
    pub notice: Signal<String>,
    /// Whether the dialog that creates a pipeline is open.
    #[get(type(copy))]
    pub create_open: Signal<bool>,
    /// The name of the pipeline being created.
    #[get(type(copy))]
    pub draft_name: Signal<String>,
    /// The description of the pipeline being created.
    #[get(type(copy))]
    pub draft_description: Signal<String>,
    /// The configuration of the pipeline being created.
    #[get(type(copy))]
    pub draft_config: Signal<String>,
    /// The detail of the open run.
    #[get(type(copy))]
    pub detail: Signal<Option<RunDetail>>,
    /// Whether the drawer of the open run is open.
    #[get(type(copy))]
    pub detail_open: Signal<bool>,
    /// The lines of the log stream of the open run.
    #[get(type(copy))]
    pub logs: Signal<Vec<LogEntry>>,
    /// The step whose log is tailed.
    #[get(type(copy))]
    pub log_step: Signal<i32>,
    /// Whether the log stream is open.
    #[get(type(copy))]
    pub streaming: Signal<bool>,
}

/// The log stream of one run.
///
/// The stream and the handlers of the stream live together, so the handlers
/// stay alive for as long as the stream does.
#[derive(Clone)]
pub(crate) struct LogStream {
    /// The stream of the run.
    pub source: EventSource,
    /// The handlers of the events of the stream.
    pub handlers: Vec<JsValue>,
}
