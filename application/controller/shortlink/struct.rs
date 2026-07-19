use super::*;

/// Route handler for resolving a short link to its original URL.
#[route("/api/shortlink/query/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct QueryRoute;

/// Route handler for creating a new short link.
#[route("/api/shortlink/insert")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct InsertRoute;
