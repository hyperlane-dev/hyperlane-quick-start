use super::*;

/// Route structure for the RSS feed view endpoints.
#[route("/rss")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct RssViewRoute;
