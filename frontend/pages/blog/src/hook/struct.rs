use super::*;

/// The reactive state the blog page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct BlogState {
    /// The tab being displayed.
    #[get(type(copy))]
    pub tab: Signal<String>,
    /// The posts of the list being displayed.
    #[get(type(copy))]
    pub posts: Signal<Vec<BlogPost>>,
    /// The number of posts the list holds in total.
    #[get(type(copy))]
    pub total: Signal<i64>,
    /// The page being displayed.
    #[get(type(copy))]
    pub page: Signal<i32>,
    /// The keyword the list is filtered by.
    #[get(type(copy))]
    pub keyword: Signal<String>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
    /// The post whose detail is open.
    #[get(type(copy))]
    pub detail: Signal<Option<BlogPost>>,
    /// The comments of the open post.
    #[get(type(copy))]
    pub comments: Signal<Vec<BlogComment>>,
    /// The body of the comment being written.
    #[get(type(copy))]
    pub comment_body: Signal<String>,
    /// The title of the post being written.
    #[get(type(copy))]
    pub draft_title: Signal<String>,
    /// The summary of the post being written.
    #[get(type(copy))]
    pub draft_summary: Signal<String>,
    /// The body of the post being written.
    #[get(type(copy))]
    pub draft_content: Signal<String>,
    /// Whether the post being written is published.
    #[get(type(copy))]
    pub draft_published: Signal<bool>,
}
