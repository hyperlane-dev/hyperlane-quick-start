use super::*;

/// The trace the visitor looks for.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TraceSearch {
    /// The identifier of the trace.
    pub trace: String,
}
