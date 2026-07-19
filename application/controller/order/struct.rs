use super::*;

/// record create route.
#[route("/api/order/record/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RecordCreateRoute;

/// record list route.
#[route("/api/order/record/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RecordListRoute;

/// record get route.
#[route("/api/order/record/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RecordGetRoute;

/// overview statistics route.
#[route("/api/order/overview/statistics")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct OverviewStatisticsRoute;

/// image upload route.
#[route("/api/order/image/upload")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ImageUploadRoute;

/// image list route.
#[route("/api/order/image/list/{record_id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ImageListRoute;

/// image download route.
#[route("/api/order/image/download/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ImageDownloadRoute;
