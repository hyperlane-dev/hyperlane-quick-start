use super::*;

#[route("/api/order/record/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RecordCreateRoute;

#[route("/api/order/record/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RecordListRoute;

#[route("/api/order/record/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RecordGetRoute;

#[route("/api/order/overview/statistics")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct OverviewStatisticsRoute;

#[route("/api/order/image/upload")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ImageUploadRoute;

#[route("/api/order/image/list/{record_id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ImageListRoute;

#[route("/api/order/image/download/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ImageDownloadRoute;
