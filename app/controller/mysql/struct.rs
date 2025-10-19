use super::*;

#[route("/api/mysql/list")]
pub struct ListRecordsRoute;

#[route("/api/mysql/create")]
pub struct CreateRecordRoute;

#[route("/api/mysql/update")]
pub struct UpdateRecordRoute;

#[route("/api/mysql/delete")]
pub struct DeleteRecordRoute;
