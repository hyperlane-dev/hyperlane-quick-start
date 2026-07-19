use super::*;

/// server status route.
#[route("/api/server/status")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ServerStatusRoute;

/// system info route.
#[route("/api/server/info")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct SystemInfoRoute;

/// network capture route.
#[route("/api/network/capture")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NetworkCaptureRoute;

/// network capture stream route.
#[route("/api/network/capture/stream")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct NetworkCaptureStreamRoute;

/// performance history route.
#[route("/api/server/performance/history")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct PerformanceHistoryRoute;
