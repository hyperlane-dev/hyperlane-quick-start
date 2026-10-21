use super::*;

/// Creates the reactive state the rss page renders from.
///
/// The first page of the feed is loaded as soon as the page is mounted.
///
/// # Returns
///
/// - `RssState` - The state the page renders from.
pub fn use_rss_state() -> RssState {
    let items: Signal<Vec<RssItem>> = App::use_signal(Vec::new);
    let title: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let description: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let page: Signal<i32> = App::use_signal(|| PAGE_FIRST);
    let page_size: Signal<i32> = App::use_signal(|| PAGE_SIZE);
    let has_more: Signal<bool> = App::use_signal(|| false);
    let loading: Signal<bool> = App::use_signal(|| true);
    let error: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let state: RssState = RssState {
        items,
        title,
        description,
        page,
        page_size,
        has_more,
        loading,
        error,
    };
    let _started: Signal<bool> = App::use_signal(|| {
        spawn_local(load_feed(state));
        true
    });
    state
}

/// Loads the page of the feed the state points at.
///
/// The feed is asked for one item more than the page holds: the extra item
/// tells the page whether a page follows the one being displayed.
///
/// # Arguments
///
/// - `RssState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The items have been written into the state.
pub async fn load_feed(state: RssState) {
    state.get_loading().set(true);
    let page: i32 = state.get_page().get();
    let page_size: i32 = state.get_page_size().get();
    let limit: i32 = page_size + 1;
    let offset: i32 = (page - PAGE_FIRST) * page_size;
    let path: String = format!("{PATH_RSS_FEED}?{QUERY_LIMIT}={limit}&{QUERY_OFFSET}={offset}");
    match request_text(HttpMethod::Get, &path, None).await {
        Ok(xml) => match parse_feed(&xml) {
            Ok(channel) => {
                let mut items: Vec<RssItem> = channel.items;
                let has_more: bool = items.len() > page_size as usize;
                if has_more {
                    items.truncate(page_size as usize);
                }
                state.get_items().set(items);
                state.get_title().set(channel.title);
                state.get_description().set(channel.description);
                state.get_has_more().set(has_more);
                state.get_error().set(TEXT_EMPTY.to_string());
            }
            Err(message) => state.get_error().set(message),
        },
        Err(error) => state.get_error().set(error.message),
    }
    state.get_loading().set(false);
}
