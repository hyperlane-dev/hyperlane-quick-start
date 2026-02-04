use super::*;

#[route("/api/redis/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ListRecordsRoute;

#[route("/api/redis/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CreateRecordRoute;

#[route("/api/redis/update")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UpdateRecordRoute;

#[route("/api/redis/delete")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct DeleteRecordRoute;
