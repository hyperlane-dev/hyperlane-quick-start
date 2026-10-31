use super::*;

/// The reactive state the rss page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default)]
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
    /// The selected page size bound to the UI control.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page_size_value: Signal<String>,
    /// Whether the feed holds a page after the one being displayed.
    #[get(type(copy))]
    pub has_more: Signal<bool>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// Whether the previous-page action is unavailable.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) previous_disabled: Signal<bool>,
    /// Whether the next-page action is unavailable.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) next_disabled: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
}
