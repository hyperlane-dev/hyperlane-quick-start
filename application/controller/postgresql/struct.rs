use super::*;

#[route("/api/postgresql/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ListRecordsRoute;

#[route("/api/postgresql/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CreateRecordRoute;

#[route("/api/postgresql/update")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UpdateRecordRoute;

#[route("/api/postgresql/delete")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct DeleteRecordRoute;
