use super::*;

#[route("/api/redis/list")]
pub struct ListRecordsRoute;

#[route("/api/redis/create")]
pub struct CreateRecordRoute;

#[route("/api/redis/update")]
pub struct UpdateRecordRoute;

#[route("/api/redis/delete")]
pub struct DeleteRecordRoute;
