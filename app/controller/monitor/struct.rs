use super::*;

#[route("/api/server/status")]
pub struct ServerStatusRoute;

#[route("/api/server/info")]
pub struct SystemInfoRoute;

#[route("/api/network/capture")]
pub struct NetworkCaptureRoute;

#[route("/api/network/capture/stream")]
pub struct NetworkCaptureStreamRoute;
