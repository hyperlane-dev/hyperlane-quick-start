use super::*;

/// trace log route.
#[route("/log/trace")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct TraceLogRoute;

/// debug log route.
#[route("/log/debug")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct DebugLogRoute;

/// info log route.
#[route("/log/info")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct InfoLogRoute;

/// warn log route.
#[route("/log/warn")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct WarnLogRoute;

/// error log route.
#[route("/log/error")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ErrorLogRoute;
