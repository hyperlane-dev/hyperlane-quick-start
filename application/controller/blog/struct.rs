use super::*;

/// blog post create route.
#[route("/api/blog/post/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostCreateRoute;

/// blog post update route.
#[route("/api/blog/post/update/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostUpdateRoute;

/// blog post delete route.
#[route("/api/blog/post/delete/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostDeleteRoute;

/// blog post get route.
#[route("/api/blog/post/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostGetRoute;

/// blog post list route.
#[route("/api/blog/post/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostListRoute;

/// blog post my list route.
#[route("/api/blog/post/my-list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostMyListRoute;

/// blog post like route.
#[route("/api/blog/post/like/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostLikeRoute;

/// blog post favorite route.
#[route("/api/blog/post/favorite/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostFavoriteRoute;

/// blog post favorite list route.
#[route("/api/blog/post/favorite-list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostFavoriteListRoute;

/// blog comment create route.
#[route("/api/blog/comment/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogCommentCreateRoute;

/// blog comment delete route.
#[route("/api/blog/comment/delete/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogCommentDeleteRoute;

/// blog comment list route.
#[route("/api/blog/comment/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogCommentListRoute;

/// blog image upload route.
#[route("/api/blog/image/upload")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogImageUploadRoute;

/// blog image download route.
#[route("/api/blog/image/download/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogImageDownloadRoute;
