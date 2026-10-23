use super::*;

/// The reactive state the trace page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct TraceState {
    /// The identifier of the trace being looked for.
    #[get(type(copy))]
    pub trace: Signal<String>,
    /// The answer of the server.
    #[get(type(copy))]
    pub result: Signal<String>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
    /// Whether the visitor has asked for a trace.
    #[get(type(copy))]
    pub searched: Signal<bool>,
}
