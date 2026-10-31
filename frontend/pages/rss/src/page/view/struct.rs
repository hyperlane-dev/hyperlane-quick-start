use super::*;

/// The props of the rss page.
#[derive(Clone, Copy, Debug, Default)]
pub struct RssPageProps {
    /// The state the page renders from.
    pub state: RssState,
}

/// The props of one item of the feed.
#[derive(Clone, Debug, Default)]
pub struct RssItemProps {
    /// The item the card shows.
    pub item: RssItem,
}
