use super::*;

/// The reactive state the monitor page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct MonitorState {
    /// The static information of the server.
    #[get(type(copy))]
    pub info: Signal<SystemInfo>,
    /// The live status of the server, absent before the first frame.
    #[get(type(copy))]
    pub status: Signal<Option<ServerStatus>>,
    /// The samples of the performance history.
    #[get(type(copy))]
    pub history: Signal<Vec<PerformancePoint>>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
    /// Whether the live status stream is connected.
    #[get(type(copy))]
    pub connected: Signal<bool>,
}

/// The live status stream of the server and the handlers bound to it.
///
/// The stream is kept alive for the lifetime of the page: dropping the
/// handle would close the connection the server holds open.
#[derive(Debug)]
pub struct MonitorStream {
    /// The stream itself.
    pub source: EventSource,
    /// The handler of the status frames.
    pub message: Closure<dyn FnMut(MessageEvent)>,
    /// The handler of the open event.
    pub open: Closure<dyn FnMut(Event)>,
    /// The handler of the error event.
    pub error: Closure<dyn FnMut(Event)>,
}
