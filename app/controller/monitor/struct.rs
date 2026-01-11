use super::*;

#[route("/api/server/status")]
#[derive(Clone, Copy, Debug, Default)]
pub struct ServerStatusRoute;

#[route("/api/server/info")]
#[derive(Clone, Copy, Debug, Default)]
pub struct SystemInfoRoute;

#[route("/api/network/capture")]
#[derive(Clone, Copy, Debug, Default)]
pub struct NetworkCaptureRoute;

#[route("/api/network/capture/stream")]
#[derive(Clone, Copy, Debug, Default)]
pub struct NetworkCaptureStreamRoute;
