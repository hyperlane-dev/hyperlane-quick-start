use super::*;

#[route("/api/shortlink/query/{id}")]
#[derive(Clone, Copy, Debug, Default)]
pub struct QueryRoute;

#[route("/api/shortlink/insert")]
#[derive(Clone, Copy, Debug, Default)]
pub struct InsertRoute;
