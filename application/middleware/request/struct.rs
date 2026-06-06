use super::*;

/// Middleware that rejects non-HTTP protocol requests at priority level 1.
#[request_middleware(1)]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct HttpRequestMiddleware;

/// Middleware that adds CORS headers to the response at priority level 2.
#[request_middleware(2)]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CrossMiddleware;

/// Middleware that sets default response headers (date, server, connection, trace, content-type) at priority level 3.
#[request_middleware(3)]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ResponseHeaderMiddleware;

/// Middleware that sets the default response status code to 200 at priority level 4.
#[request_middleware(4)]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ResponseStatusCodeMiddleware;

/// Middleware that handles HTTP OPTIONS preflight requests at priority level 5.
#[request_middleware(5)]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct OptionMethodMiddleware;

/// Middleware that handles WebSocket upgrade requests at priority level 6.
#[request_middleware(6)]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UpgradeMiddleware;
