use super::*;

#[route("/api/blog/post/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostCreateRoute;

#[route("/api/blog/post/update/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostUpdateRoute;

#[route("/api/blog/post/delete/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostDeleteRoute;

#[route("/api/blog/post/get/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostGetRoute;

#[route("/api/blog/post/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostListRoute;

#[route("/api/blog/post/my-list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostMyListRoute;

#[route("/api/blog/post/like/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostLikeRoute;

#[route("/api/blog/post/favorite/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostFavoriteRoute;

#[route("/api/blog/post/favorite-list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostFavoriteListRoute;

#[route("/api/blog/comment/create")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogCommentCreateRoute;

#[route("/api/blog/comment/delete/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogCommentDeleteRoute;

#[route("/api/blog/comment/list")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogCommentListRoute;

#[route("/api/blog/image/upload")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogImageUploadRoute;

#[route("/api/blog/image/download/{id}")]
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogImageDownloadRoute;
