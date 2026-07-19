use super::*;

/// list records route.
#[route("/api/redis/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct ListRecordsRoute;

/// create record route.
#[route("/api/redis/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct CreateRecordRoute;

/// update record route.
#[route("/api/redis/update")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct UpdateRecordRoute;

/// delete record route.
#[route("/api/redis/delete")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct DeleteRecordRoute;
