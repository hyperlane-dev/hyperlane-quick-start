use super::*;

/// Route handler for Server-Sent Events (SSE) streaming endpoint.
#[route("/sse")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct SseRoute;
