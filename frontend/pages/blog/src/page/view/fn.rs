use super::*;

/// Renders the blog page.
///
/// The page is a tab bar over the three post lists and the editor, with the
/// detail of a post opened in a modal above them.
///
/// # Arguments
///
/// - `VirtualNode<BlogPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn blog_page(node: VirtualNode<BlogPageProps>) -> VirtualNode {
    let BlogPageProps { state }: BlogPageProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "📝"
                title: "Hyperlane Blog"
                subtitle: "Publish, read and discuss posts."
            }
            euv_tabs {
                items: TAB_ITEMS.to_vec()
                active: state.get_tab()
            }
            if { state.get_tab().get() == TAB_EXPLORE } {
                post_list_panel {
                    state: state
                }
            } else if { state.get_tab().get() == TAB_MY_POSTS } {
                post_list_panel {
                    state: state
                }
            } else if { state.get_tab().get() == TAB_FAVORITES } {
                post_list_panel {
                    state: state
                }
            } else {
                write_panel {
                    state: state
                }
            }
            if { state.get_detail().get().is_some() } {
                euv_modal {
                    title: TITLE_DETAIL
                    onclick: close_detail_handler(state)
                    post_detail {
                        state: state
                    }
                }
            }
        }
    }
}

/// Renders the list of posts of the tab being displayed.
///
/// The list carries the three metrics of the posts it shows, the search field
/// and the pager that walks through the pages of the list.
///
/// # Arguments
///
/// - `VirtualNode<PostListPanelProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The list panel tree.
#[component]
pub fn post_list_panel(node: VirtualNode<PostListPanelProps>) -> VirtualNode {
    let PostListPanelProps { state }: PostListPanelProps = node.try_get_props().unwrap_or_default();
    let posts: Signal<Vec<BlogPost>> = state.get_posts();
    html! {
        div {
            class: c_blog_stats()
            if { !state.get_posts().get().is_empty() } {
                stat_tile {
                    label: LABEL_POSTS
                    value: state.get_total().get().to_string()
                    icon: ICON_POSTS
                }
                stat_tile {
                    label: LABEL_LIKES
                    value: total_likes(&state.get_posts().get()).to_string()
                    icon: ICON_LIKES
                }
                stat_tile {
                    label: LABEL_COMMENTS
                    value: total_comments(&state.get_posts().get()).to_string()
                    icon: ICON_COMMENTS
                }
            }
        }
        euv_card {
            title: TITLE_POSTS
            div {
                class: c_blog_toolbar()
                euv_input {
                    id: ID_SEARCH
                    label: LABEL_SEARCH
                    placeholder: PLACEHOLDER_SEARCH
                    value: state.get_keyword()
                    oninput: search_handler(state)
                }
            }
            if { state.get_loading().get() } {
                euv_loading {
                    title: "Loading the posts"
                }
            } else if { !state.get_error().get().is_empty() } {
                euv_alert {
                    variant: AlertVariant::Error
                    state.get_error().get().as_str()
                }
            } else if { state.get_posts().get().is_empty() } {
                euv_result {
                    code: "0"
                    title: TITLE_EMPTY
                    description: TEXT_EMPTY
                }
            } else {
                div {
                    class: c_blog_posts()
                    for post in { posts } {
                        post_row {
                            state: state
                            post: post
                        }
                    }
                }
                pagination_bar {
                    state: state
                }
            }
        }
    }
}

/// Renders one metric tile of the post list.
///
/// # Arguments
///
/// - `VirtualNode<StatTileProps>` - The props node carrying the metric.
///
/// # Returns
///
/// - `VirtualNode` - The metric tile tree.
#[component]
pub fn stat_tile(node: VirtualNode<StatTileProps>) -> VirtualNode {
    let StatTileProps {
        label,
        value,
        icon,
    }: StatTileProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_hls_stat_card()
            div {
                class: c_hls_stat_icon()
                icon
            }
            div {
                class: c_hls_stat_value()
                value
            }
            div {
                class: c_hls_stat_label()
                label
            }
        }
    }
}

/// Renders one post of the list.
///
/// # Arguments
///
/// - `VirtualNode<PostRowProps>` - The props node carrying the post.
///
/// # Returns
///
/// - `VirtualNode` - The post row tree.
#[component]
pub fn post_row(node: VirtualNode<PostRowProps>) -> VirtualNode {
    let PostRowProps { state, post }: PostRowProps = node.try_get_props().unwrap_or_default();
    let post_id: String = post.id.clone();
    let author: String = post.username.clone().unwrap_or_default();
    let summary: String = post.summary.clone().unwrap_or_default();
    let time: String = relative_time(post.created_at);
    html! {
        div {
            class: c_blog_post()
            div {
                class: c_blog_post_header()
                span {
                    class: c_blog_post_author()
                    author.as_str()
                }
                span {
                    class: c_blog_post_time()
                    time.as_str()
                }
            }
            h3 {
                class: c_blog_post_title()
                post.title.as_str()
            }
            if !summary.is_empty() {
                p {
                    class: c_blog_post_summary()
                    summary.as_str()
                }
            }
            div {
                class: c_blog_post_stats()
                euv_button {
                    variant: EuvButtonVariant::Primary
                    onclick: open_post_handler(state, post_id.clone())
                    LABEL_OPEN
                }
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: like_handler(state, post_id.clone())
                    LABEL_LIKE
                }
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: favorite_handler(state, post_id.clone())
                    LABEL_FAVORITE
                }
                span {
                    class: c_blog_post_count()
                    post.like_count
                }
                span {
                    class: c_blog_post_count()
                    post.comment_count
                }
            }
        }
    }
}

/// Renders the pager of the post list.
///
/// # Arguments
///
/// - `VirtualNode<PostListPanelProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The pager tree.
#[component]
pub fn pagination_bar(node: VirtualNode<PostListPanelProps>) -> VirtualNode {
    let PostListPanelProps { state }: PostListPanelProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_blog_pagination()
            euv_button {
                variant: EuvButtonVariant::Outline
                onclick: previous_page_handler(state)
                LABEL_PREVIOUS
            }
            span {
                class: c_blog_pagination_info()
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

/// Renders the editor of a new post.
///
/// # Arguments
///
/// - `VirtualNode<WritePanelProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The editor tree.
#[component]
pub fn write_panel(node: VirtualNode<WritePanelProps>) -> VirtualNode {
    let WritePanelProps { state }: WritePanelProps = node.try_get_props().unwrap_or_default();
    html! {
        euv_card {
            title: TITLE_WRITE
            div {
                class: c_blog_form()
                euv_field {
                    id: ID_TITLE
                    label: LABEL_TITLE
                    placeholder: PLACEHOLDER_TITLE
                    value: state.get_draft_title()
                    oninput: draft_title_handler(state)
                }
                euv_field {
                    id: ID_SUMMARY
                    label: LABEL_SUMMARY
                    placeholder: PLACEHOLDER_SUMMARY
                    value: state.get_draft_summary()
                    oninput: draft_summary_handler(state)
                }
                div {
                    class: c_form_label()
                    LABEL_CONTENT
                }
                textarea {
                    class: c_textarea_input()
                    placeholder: PLACEHOLDER_CONTENT
                    rows: ROWS_CONTENT
                    oninput: draft_content_handler(state)
                }
                div {
                    class: c_button_controls()
                    euv_switch {
                        id: ID_PUBLISHED
                        label: LABEL_PUBLISHED
                        checked: state.get_draft_published()
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        onclick: publish_handler(state)
                        LABEL_PUBLISH
                    }
                }
            }
        }
    }
}

/// Renders the detail of the open post and its comments.
///
/// # Arguments
///
/// - `VirtualNode<PostDetailProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The detail tree.
#[component]
pub fn post_detail(node: VirtualNode<PostDetailProps>) -> VirtualNode {
    let PostDetailProps { state }: PostDetailProps = node.try_get_props().unwrap_or_default();
    let post: Option<BlogPost> = state.get_detail().get();
    let Some(post) = post else {
        return html! {
            ""
        };
    };
    let author: String = post.username.clone().unwrap_or_default();
    let time: String = relative_time(post.created_at);
    let comments: Signal<Vec<BlogComment>> = state.get_comments();
    html! {
        div {
            class: c_blog_detail()
            div {
                class: c_blog_post_header()
                span {
                    class: c_blog_post_author()
                    author.as_str()
                }
                span {
                    class: c_blog_post_time()
                    time.as_str()
                }
            }
            h3 {
                class: c_blog_post_title()
                post.title.as_str()
            }
            pre {
                class: c_blog_post_body()
                post.content.as_str()
            }
            div {
                class: c_blog_comments()
                for comment in { comments } {
                    div {
                        class: c_blog_comment()
                        span {
                            class: c_blog_post_author()
                            comment.username.as_str()
                        }
                        p {
                            class: c_blog_post_summary()
                            comment.content.as_str()
                        }
                    }
                }
            }
            div {
                class: c_blog_form()
                div {
                    class: c_form_label()
                    LABEL_COMMENT
                }
                textarea {
                    id: ID_COMMENT
                    class: c_textarea_input()
                    placeholder: PLACEHOLDER_COMMENT
                    oninput: comment_handler(state)
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        onclick: send_comment_handler(state)
                        LABEL_SEND
                    }
                }
            }
        }
    }
}

/// Sums the likes of the posts of a list.
///
/// # Arguments
///
/// - `&[BlogPost]` - The posts of the list.
///
/// # Returns
///
/// - `i32` - The number of likes the list holds.
fn total_likes(posts: &[BlogPost]) -> i32 {
    posts.iter().map(|post: &BlogPost| post.like_count).sum()
}

/// Sums the comments of the posts of a list.
///
/// # Arguments
///
/// - `&[BlogPost]` - The posts of the list.
///
/// # Returns
///
/// - `i32` - The number of comments the list holds.
fn total_comments(posts: &[BlogPost]) -> i32 {
    posts.iter().map(|post: &BlogPost| post.comment_count).sum()
}

/// Formats the moment a post was written as a short relative time.
///
/// # Arguments
///
/// - `i64` - The moment in milliseconds since the Unix epoch.
///
/// # Returns
///
/// - `String` - The relative time of the moment.
fn relative_time(timestamp: i64) -> String {
    let now: i64 = js_sys::Date::now() as i64;
    let elapsed: i64 = now - timestamp;
    if elapsed < MILLIS_MINUTE {
        return TEXT_JUST_NOW.to_string();
    }
    let minutes: i64 = elapsed / MILLIS_MINUTE;
    if minutes < MINUTES_HOUR {
        return format!("{minutes}{TEXT_MINUTE_AGO}");
    }
    let hours: i64 = minutes / MINUTES_HOUR;
    if hours < HOURS_DAY {
        return format!("{hours}{TEXT_HOUR_AGO}");
    }
    format!("{}{TEXT_DAY_AGO}", hours / HOURS_DAY)
}

/// Builds the handler that reloads the list with the keyword of the search field.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn search_handler(state: BlogState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let value: String = input_value(&event);
        state.get_keyword().set(value);
        state.get_page().set(PAGE_FIRST);
        spawn_local(load_posts(state));
    }))
}

/// Builds the handler that opens the detail of a post.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
/// - `String` - The identifier of the post.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn open_post_handler(state: BlogState, id: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(open_post(state, id.clone()));
    }))
}

/// Builds the handler that likes a post.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
/// - `String` - The identifier of the post.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn like_handler(state: BlogState, id: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(toggle_like(state, id.clone()));
    }))
}

/// Builds the handler that favorites a post.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
/// - `String` - The identifier of the post.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn favorite_handler(state: BlogState, id: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(toggle_favorite(state, id.clone()));
    }))
}

/// Builds the handler that closes the detail of a post.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_detail_handler(state: BlogState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_detail().set(None);
    }))
}

/// Builds the handler that shows the previous page of the list.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn previous_page_handler(state: BlogState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let page: i32 = state.get_page().get();
        if page > PAGE_FIRST {
            state.get_page().set(page - 1);
            spawn_local(load_posts(state));
        }
    }))
}

/// Builds the handler that shows the next page of the list.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn next_page_handler(state: BlogState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let page: i32 = state.get_page().get();
        let total: i64 = state.get_total().get();
        let last: i32 = ((total + PAGE_SIZE as i64 - 1) / PAGE_SIZE as i64) as i32;
        if page < last {
            state.get_page().set(page + 1);
            spawn_local(load_posts(state));
        }
    }))
}

/// Builds the handler that keeps the title of the post being written.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn draft_title_handler(state: BlogState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_draft_title().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the summary of the post being written.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn draft_summary_handler(state: BlogState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_draft_summary().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the body of the post being written.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn draft_content_handler(state: BlogState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_draft_content().set(textarea_value(&event));
    }))
}

/// Builds the handler that publishes the post being written.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn publish_handler(state: BlogState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(publish_post(state));
    }))
}

/// Builds the handler that keeps the comment being written.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn comment_handler(state: BlogState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_comment_body().set(textarea_value(&event));
    }))
}

/// Builds the handler that sends the comment being written.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn send_comment_handler(state: BlogState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(submit_comment(state));
    }))
}
