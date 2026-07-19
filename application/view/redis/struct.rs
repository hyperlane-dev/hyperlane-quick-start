use super::*;

/// Route structure for the Redis management view endpoints.
#[route("/redis")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RedisViewRoute;
