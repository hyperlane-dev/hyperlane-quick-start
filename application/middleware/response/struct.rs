use super::*;

/// Middleware that sends the response body at priority level 1.
#[response_middleware(1)]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct SendMiddleware;

/// Middleware that logs the request and response details at priority level 2.
#[response_middleware(2)]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct LogMiddleware;
