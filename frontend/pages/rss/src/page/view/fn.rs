use super::*;

/// Renders the rss page.
///
/// The page shows the channel of the feed, the items of the page being
/// displayed and the controls that walk through the pages of the feed.
///
/// # Arguments
///
/// - `VirtualNode<RssPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn rss_page(node: VirtualNode<RssPageProps>) -> VirtualNode {
    let RssPageProps { state }: RssPageProps = node.try_get_props().unwrap_or_default();
    let items: Signal<Vec<RssItem>> = state.get_items();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_RSS
                title: TITLE_RSS
                subtitle: SUBTITLE_RSS
            }
            if { !state.get_error().get().is_empty() } {
                euv_alert {
                    variant: AlertVariant::Error
                    state.get_error().get().as_str()
                }
            }
            euv_card {
                title: TITLE_CHANNEL
                div {
                    class: c_rss_channel()
                    h3 {
                        class: c_rss_channel_title()
                        state.get_title().get().as_str()
                    }
                    p {
                        class: c_rss_channel_description()
                        state.get_description().get().as_str()
                    }
                }
            }
            euv_card {
                title: TITLE_ITEMS
                if { state.get_loading().get() } {
                    euv_loading {
                        title: TITLE_LOADING
                    }
                } else if { items.get().is_empty() } {
                    euv_result {
                        code: CODE_EMPTY
                        title: TITLE_EMPTY
                        description: TEXT_NO_ITEM
                    }
                } else {
                    div {
                        class: c_rss_items()
                        for item in { items } {
                            rss_item {
                                item: item
                            }
                        }
                    }
                }
            }
            div {
                class: c_rss_controls()
                div {
                    class: c_rss_page_size()
                    label {
                        class: c_form_label()
                        LABEL_PAGE_SIZE
                    }
                    select {
                        class: c_select_input()
                        value: state.get_page_size().get().to_string()
                        onchange: page_size_handler(state)
                        for &size in PAGE_SIZES {
                            option {
                                value: size
                                size
                            }
                        }
                    }
                }
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: refresh_handler(state)
                    LABEL_REFRESH
                }
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: previous_page_handler(state)
                    LABEL_PREVIOUS
                }
                span {
                    class: c_rss_pagination_info()
                    state.get_page().get().to_string()
                }
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: next_page_handler(state)
                    LABEL_NEXT
                }
            }
        }
    }
}

/// Renders one item of the feed.
///
/// # Arguments
///
/// - `VirtualNode<RssItemProps>` - The props node carrying the item.
///
/// # Returns
///
/// - `VirtualNode` - The item tree.
#[component]
pub fn rss_item(node: VirtualNode<RssItemProps>) -> VirtualNode {
    let RssItemProps { item }: RssItemProps = node.try_get_props().unwrap_or_default();
    let title: String = item.title.clone();
    let link: String = item.link.clone();
    let description: String = item.description.clone();
    let pub_date: String = item.pub_date.clone();
    let (download, size, file_type) = match item.enclosure.clone() {
        Some(enclosure) => (
            enclosure.url,
            enclosure.length,
            enclosure.file_type,
        ),
        None => (String::new(), String::new(), String::new()),
    };
    let has_download: bool = !download.is_empty();
    html! {
        euv_card {
            div {
                class: c_rss_item()
                a {
                    class: c_rss_item_title()
                    href: link.clone()
                    target: TARGET_BLANK
                    title.as_str()
                }
                p {
                    class: c_rss_item_description()
                    description.as_str()
                }
                div {
                    class: c_rss_item_meta()
                    span {
                        class: c_rss_item_date()
                        pub_date.as_str()
                    }
                    span {
                        class: c_rss_item_size()
                        size.as_str()
                    }
                    span {
                        class: c_rss_item_type()
                        file_type.as_str()
                    }
                }
                if { has_download == true } {
                    a {
                        class: c_rss_item_download()
                        href: download.clone()
                        target: TARGET_BLANK
                        LABEL_DOWNLOAD
                    }
                }
            }
        }
    }
}

/// Builds the handler that keeps the number of the items one page holds.
///
/// # Arguments
///
/// - `RssState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The change handler.
fn page_size_handler(state: RssState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let value: String = select_value(&event);
        let Ok(size) = value.parse::<i32>() else {
            return;
        };
        state.get_page_size().set(size);
        state.get_page().set(PAGE_FIRST);
        spawn_local(load_feed(state));
    }))
}

/// Builds the handler that reloads the page of the feed.
///
/// # Arguments
///
/// - `RssState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn refresh_handler(state: RssState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(load_feed(state));
    }))
}

/// Builds the handler that shows the previous page of the feed.
///
/// # Arguments
///
/// - `RssState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn previous_page_handler(state: RssState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let page: i32 = state.get_page().get();
        if page > PAGE_FIRST {
            state.get_page().set(page - 1);
            spawn_local(load_feed(state));
        }
    }))
}

/// Builds the handler that shows the next page of the feed.
///
/// # Arguments
///
/// - `RssState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn next_page_handler(state: RssState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let page: i32 = state.get_page().get();
        if state.get_has_more().get() {
            state.get_page().set(page + 1);
            spawn_local(load_feed(state));
        }
    }))
}

/// Reads the value of the select the event came from.
///
/// # Arguments
///
/// - `&Event` - The event the value is read from.
///
/// # Returns
///
/// - `String` - The value of the select, empty when the event carried none.
fn select_value(event: &Event) -> String {
    let Some(target): Option<EventTarget> = event.target() else {
        return String::new();
    };
    let Ok(element): Result<HtmlSelectElement, EventTarget> = target.dyn_into() else {
        return String::new();
    };
    element.value()
}
