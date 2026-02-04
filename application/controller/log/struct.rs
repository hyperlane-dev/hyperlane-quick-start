use super::*;

#[route("/log/trace")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct TraceLogRoute;

#[route("/log/debug")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct DebugLogRoute;

#[route("/log/info")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct InfoLogRoute;

#[route("/log/warn")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct WarnLogRoute;

#[route("/log/error")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ErrorLogRoute;
