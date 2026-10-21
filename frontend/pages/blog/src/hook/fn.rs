use super::*;

/// Creates the reactive state the blog page renders from.
///
/// The state holds the list that is being displayed, the post whose detail
/// is open and the drafts of the editor. The first list is loaded as soon as
/// the page is mounted, and every later tab change loads the list of the tab
/// that is entered.
///
/// # Returns
///
/// - `BlogState` - The state the page renders from.
pub fn use_blog_state() -> BlogState {
    let tab: Signal<String> = App::use_signal(|| TAB_EXPLORE.to_string());
    let posts: Signal<Vec<BlogPost>> = App::use_signal(Vec::new);
    let total: Signal<i64> = App::use_signal(|| 0);
    let page: Signal<i32> = App::use_signal(|| PAGE_FIRST);
    let keyword: Signal<String> = App::use_signal(|| DRAFT_EMPTY.to_string());
    let loading: Signal<bool> = App::use_signal(|| true);
    let error: Signal<String> = App::use_signal(|| DRAFT_EMPTY.to_string());
    let detail: Signal<Option<BlogPost>> = App::use_signal(|| None);
    let comments: Signal<Vec<BlogComment>> = App::use_signal(Vec::new);
    let comment_body: Signal<String> = App::use_signal(|| COMMENT_EMPTY.to_string());
    let draft_title: Signal<String> = App::use_signal(|| DRAFT_EMPTY.to_string());
    let draft_summary: Signal<String> = App::use_signal(|| DRAFT_EMPTY.to_string());
    let draft_content: Signal<String> = App::use_signal(|| DRAFT_EMPTY.to_string());
    let draft_published: Signal<bool> = App::use_signal(|| true);
    let state: BlogState = BlogState {
        tab,
        posts,
        total,
        page,
        keyword,
        loading,
        error,
        detail,
        comments,
        comment_body,
        draft_title,
        draft_summary,
        draft_content,
        draft_published,
    };
    let _started: Signal<bool> = App::use_signal(|| {
        spawn_local(load_posts(state));
        true
    });
    watch!(tab, |tab_value: String| {
        state.get_page().set(PAGE_FIRST);
        if tab_value == TAB_WRITE {
            return;
        }
        spawn_local(load_posts(state));
    });
    state
}

/// Loads the posts of the list being displayed.
///
/// The endpoint follows the tab: the published posts, the posts of the signed
/// in visitor or the posts the visitor favorited.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The list has been written into the state.
pub async fn load_posts(state: BlogState) {
    let tab: String = state.get_tab().get();
    if tab == TAB_WRITE {
        return;
    }
    let page: i32 = state.get_page().get();
    let keyword: String = state.get_keyword().get();
    state.get_loading().set(true);
    let path: String = posts_path(&tab, page, &keyword);
    match get_json::<BlogPostList>(&path).await {
        Ok(list) => {
            state.get_posts().set(list.posts);
            state.get_total().set(list.total);
            state.get_error().set(DRAFT_EMPTY.to_string());
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
    state.get_loading().set(false);
}

/// Opens the detail of a post and loads its comments.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
/// - `String` - The identifier of the post.
///
/// # Returns
///
/// - `()` - The detail and the comments have been written into the state.
pub async fn open_post(state: BlogState, id: String) {
    let path: String = format!("{PATH_POST_GET}/{id}");
    match get_json::<BlogPost>(&path).await {
        Ok(post) => {
            state.get_detail().set(Some(post));
            state.get_comments().set(Vec::new());
            let comment_path: String =
                format!("{PATH_COMMENT_LIST}?post_id={id}&page={PAGE_FIRST}&limit={COMMENT_LIMIT}");
            match get_json::<BlogCommentList>(&comment_path).await {
                Ok(list) => state.get_comments().set(list.comments),
                Err(error) => state.get_error().set(error.message),
            }
        }
        Err(error) => state.get_error().set(error.message),
    }
}

/// Likes or unlikes a post and reloads the list.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
/// - `String` - The identifier of the post.
///
/// # Returns
///
/// - `()` - The list has been written into the state.
pub async fn toggle_like(state: BlogState, id: String) {
    let path: String = format!("{PATH_POST_LIKE}/{id}");
    match request_json::<serde_json::Value>(HttpMethod::Post, &path, None).await {
        Ok(_) => load_posts(state).await,
        Err(error) => state.get_error().set(error.message),
    }
}

/// Favorites or unfavorites a post and reloads the list.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
/// - `String` - The identifier of the post.
///
/// # Returns
///
/// - `()` - The list has been written into the state.
pub async fn toggle_favorite(state: BlogState, id: String) {
    let path: String = format!("{PATH_POST_FAVORITE}/{id}");
    match request_json::<serde_json::Value>(HttpMethod::Post, &path, None).await {
        Ok(_) => load_posts(state).await,
        Err(error) => state.get_error().set(error.message),
    }
}

/// Publishes the post the editor holds.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The post has been published and the editor has been cleared.
pub async fn publish_post(state: BlogState) {
    let title: String = state.get_draft_title().get();
    let summary: String = state.get_draft_summary().get();
    let content: String = state.get_draft_content().get();
    if title.trim().is_empty() || content.trim().is_empty() {
        state
            .get_error()
            .set("A post needs a title and a body".to_string());
        return;
    }
    let body: PostDraft = PostDraft {
        title,
        summary: Some(summary),
        content,
        cover_image_id: None,
        is_published: state.get_draft_published().get(),
        image_ids: Vec::new(),
    };
    match post_json::<BlogPost, PostDraft>(PATH_POST_CREATE, &body).await {
        Ok(_) => {
            state.get_draft_title().set(DRAFT_EMPTY.to_string());
            state.get_draft_summary().set(DRAFT_EMPTY.to_string());
            state.get_draft_content().set(DRAFT_EMPTY.to_string());
            state.get_error().set(DRAFT_EMPTY.to_string());
            state.get_tab().set(TAB_EXPLORE.to_string());
        }
        Err(error) => state.get_error().set(error.message),
    }
}

/// Sends the comment the visitor wrote.
///
/// # Arguments
///
/// - `BlogState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The comment has been sent and the comments have been reloaded.
pub async fn submit_comment(state: BlogState) {
    let Some(post): Option<BlogPost> = state.get_detail().get() else {
        return;
    };
    let content: String = state.get_comment_body().get();
    if content.trim().is_empty() {
        return;
    }
    let body: CommentDraft = CommentDraft {
        post_id: post.id.clone(),
        parent_id: None,
        content,
    };
    match post_json::<BlogComment, CommentDraft>(PATH_COMMENT_CREATE, &body).await {
        Ok(_) => {
            state.get_comment_body().set(COMMENT_EMPTY.to_string());
            open_post(state, post.id.clone()).await;
        }
        Err(error) => state.get_error().set(error.message),
    }
}

/// Builds the path of the list of a tab.
///
/// # Arguments
///
/// - `&str` - The tab the list belongs to.
/// - `i32` - The page the list holds.
/// - `&str` - The keyword the list is filtered by.
///
/// # Returns
///
/// - `String` - The path of the list.
fn posts_path(tab: &str, page: i32, keyword: &str) -> String {
    let base: &str = match tab {
        TAB_MY_POSTS => PATH_POST_MY_LIST,
        TAB_FAVORITES => PATH_POST_FAVORITE_LIST,
        _ => PATH_POST_LIST,
    };
    let mut path: String = format!("{base}?page={page}&limit={PAGE_SIZE}");
    if !keyword.is_empty() {
        path.push_str(&format!("&keyword={}", encode_uri_component(keyword)));
    }
    path
}
