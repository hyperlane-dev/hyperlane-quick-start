use super::*;

#[route("/log/info")]
pub struct InfoLogRoute;

#[route("/log/warn")]
pub struct WarnLogRoute;

#[route("/log/error")]
pub struct ErrorLogRoute;
