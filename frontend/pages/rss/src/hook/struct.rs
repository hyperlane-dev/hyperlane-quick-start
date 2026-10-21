use super::*;

/// The reactive state the rss page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct RssState {
    /// The items of the page being displayed.
    #[get(type(copy))]
    pub items: Signal<Vec<RssItem>>,
    /// The title of the channel of the feed.
    #[get(type(copy))]
    pub title: Signal<String>,
    /// The description of the channel of the feed.
    #[get(type(copy))]
    pub description: Signal<String>,
    /// The page being displayed.
    #[get(type(copy))]
    pub page: Signal<i32>,
    /// The number of the items one page holds.
    #[get(type(copy))]
    pub page_size: Signal<i32>,
    /// Whether the feed holds a page after the one being displayed.
    #[get(type(copy))]
    pub has_more: Signal<bool>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
}
