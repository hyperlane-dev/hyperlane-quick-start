use super::*;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogPostRepository;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogCommentRepository;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogLikeRepository;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogFavoriteRepository;

#[derive(Clone, Copy, Data, Debug, Default)]
pub struct BlogImageRepository;

#[derive(Clone, Data, Debug, Default)]
pub struct BlogPostQuery {
    #[get(type(copy))]
    pub(super) user_id: Option<i32>,
    pub(super) keyword: Option<String>,
    #[get(type(copy))]
    pub(super) is_published: Option<bool>,
    #[get(type(copy))]
    pub(super) page: i32,
    #[get(type(copy))]
    pub(super) limit: u64,
}

#[derive(Clone, Data, Debug, Default)]
pub struct BlogCommentQuery {
    #[get(type(copy))]
    pub post_id: i32,
    #[get(type(copy))]
    pub page: i32,
    #[get(type(copy))]
    pub limit: u64,
}
