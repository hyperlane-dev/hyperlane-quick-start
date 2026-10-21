use super::*;

/// The props of the trace page.
#[derive(Clone, Copy, Debug, Default)]
pub struct TracePageProps {
    /// The state the page renders from.
    pub state: TraceState,
}
