use super::*;

#[route("/log/info")]
#[derive(Clone, Copy, Debug, Default)]
pub struct InfoLogRoute;

#[route("/log/warn")]
#[derive(Clone, Copy, Debug, Default)]
pub struct WarnLogRoute;

#[route("/log/error")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ErrorLogRoute;
