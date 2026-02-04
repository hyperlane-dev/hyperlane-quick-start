use super::*;

#[route("/api/trace/{trace}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct TraceRoute;
