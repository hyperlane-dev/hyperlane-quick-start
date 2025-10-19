use super::*;

#[route("/api/postgresql/list")]
pub struct ListRecordsRoute;

#[route("/api/postgresql/create")]
pub struct CreateRecordRoute;

#[route("/api/postgresql/update")]
pub struct UpdateRecordRoute;

#[route("/api/postgresql/delete")]
pub struct DeleteRecordRoute;
