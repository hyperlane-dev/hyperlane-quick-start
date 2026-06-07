use super::*;

/// Route handler for searching application log traces by trace ID.
#[route("/api/trace/{trace}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct TraceRoute;
