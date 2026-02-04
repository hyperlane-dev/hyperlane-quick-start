use super::*;

#[route("/api/tracking/report")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct TrackingReportRoute;

#[route("/api/tracking/query")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct TrackingQueryRoute;
