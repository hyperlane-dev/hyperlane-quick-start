use super::*;

/// blog post repository.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostRepository;

/// blog comment repository.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogCommentRepository;

/// blog like repository.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogLikeRepository;

/// blog favorite repository.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogFavoriteRepository;

/// blog image repository.
#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogImageRepository;

/// blog post query.
#[derive(Clone, Data, Debug, Default)]
pub struct BlogPostQuery {
    /// The user id.
    #[get(type(copy))]
    pub(super) user_id: Option<i32>,
    /// The keyword.
    pub(super) keyword: Option<String>,
    /// The is published.
    #[get(type(copy))]
    pub(super) is_published: Option<bool>,
    /// The page.
    #[get(type(copy))]
    pub(super) page: i32,
    /// The limit.
    #[get(type(copy))]
    pub(super) limit: u64,
}

/// blog comment query.
#[derive(Clone, Data, Debug, Default)]
pub struct BlogCommentQuery {
    /// The post id.
    #[get(type(copy))]
    pub post_id: i32,
    /// The page.
    #[get(type(copy))]
    pub page: i32,
    /// The limit.
    #[get(type(copy))]
    pub limit: u64,
}
