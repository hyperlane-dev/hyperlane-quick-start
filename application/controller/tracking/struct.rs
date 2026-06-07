use super::*;

/// tracking report route.
#[route("/api/tracking/report")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct TrackingReportRoute;

/// tracking query route.
#[route("/api/tracking/query")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct TrackingQueryRoute;
