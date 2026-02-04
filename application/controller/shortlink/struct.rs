use super::*;

#[route("/api/shortlink/query/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct QueryRoute;

#[route("/api/shortlink/insert")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct InsertRoute;
