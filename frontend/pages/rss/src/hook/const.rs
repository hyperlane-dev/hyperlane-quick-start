/// The endpoint of the feed of the server.
pub(crate) const PATH_RSS_FEED: &str = "/api/rss/feed";

/// The query parameter that names the number of the items.
pub(crate) const QUERY_LIMIT: &str = "limit";

/// The query parameter that names the first item of the page.
pub(crate) const QUERY_OFFSET: &str = "offset";

/// The first page of the feed.
pub(crate) const PAGE_FIRST: i32 = 1;

/// The number of the items one page holds.
pub(crate) const PAGE_SIZE: i32 = 20;

/// The empty text of a field.
pub(crate) const TEXT_EMPTY: &str = "";
