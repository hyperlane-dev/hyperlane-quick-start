/// The tab that lists the published posts.
pub(crate) const TAB_EXPLORE: &str = "explore";

/// The tab that lists the posts of the signed in visitor.
pub(crate) const TAB_MY_POSTS: &str = "my-posts";

/// The tab that lists the favorited posts.
pub(crate) const TAB_FAVORITES: &str = "favorites";

/// The tab that holds the editor.
pub(crate) const TAB_WRITE: &str = "write";

/// The number of posts one page holds.
pub(crate) const PAGE_SIZE: i32 = 10;

/// The first page of a list.
pub(crate) const PAGE_FIRST: i32 = 1;

/// The endpoint of the published posts.
pub(crate) const PATH_POST_LIST: &str = "/api/blog/post/list";

/// The endpoint of the posts of the signed in visitor.
pub(crate) const PATH_POST_MY_LIST: &str = "/api/blog/post/my-list";

/// The endpoint of the favorited posts.
pub(crate) const PATH_POST_FAVORITE_LIST: &str = "/api/blog/post/favorite-list";

/// The endpoint of one post.
pub(crate) const PATH_POST_GET: &str = "/api/blog/post/get";

/// The endpoint that creates a post.
pub(crate) const PATH_POST_CREATE: &str = "/api/blog/post/create";

/// The endpoint that likes a post.
pub(crate) const PATH_POST_LIKE: &str = "/api/blog/post/like";

/// The endpoint that favorites a post.
pub(crate) const PATH_POST_FAVORITE: &str = "/api/blog/post/favorite";

/// The endpoint of the comments of a post.
pub(crate) const PATH_COMMENT_LIST: &str = "/api/blog/comment/list";

/// The endpoint that creates a comment.
pub(crate) const PATH_COMMENT_CREATE: &str = "/api/blog/comment/create";

/// The number of comments loaded with a post.
pub(crate) const COMMENT_LIMIT: i32 = 50;

/// The body of a post that is written.
pub(crate) const DRAFT_EMPTY: &str = "";

/// The body of a comment that is written.
pub(crate) const COMMENT_EMPTY: &str = "";
