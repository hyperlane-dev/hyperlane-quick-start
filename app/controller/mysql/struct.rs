use super::*;

#[route("/api/mysql/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ListRecordsRoute;

#[route("/api/mysql/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CreateRecordRoute;

#[route("/api/mysql/update")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UpdateRecordRoute;

#[route("/api/mysql/delete")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct DeleteRecordRoute;
