use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogPostResponse {
    pub(super) id: String,
    pub(super) user_id: String,
    pub(super) username: Option<String>,
    pub(super) title: String,
    pub(super) summary: Option<String>,
    pub(super) content: String,
    pub(super) cover_image: Option<BlogImageResponse>,
    pub(super) images: Vec<BlogImageResponse>,
    #[get(type(copy))]
    pub(super) is_published: bool,
    #[get(type(copy))]
    pub(super) view_count: i32,
    #[get(type(copy))]
    pub(super) like_count: i32,
    #[get(type(copy))]
    pub(super) favorite_count: i32,
    #[get(type(copy))]
    pub(super) comment_count: i32,
    #[get(type(copy))]
    pub(super) is_liked: bool,
    #[get(type(copy))]
    pub(super) is_favorited: bool,
    #[get(type(copy))]
    pub(super) created_at: i64,
    #[get(type(copy))]
    pub(super) updated_at: i64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogPostListResponse {
    pub(super) posts: Vec<BlogPostResponse>,
    #[get(type(copy))]
    pub(super) total: i64,
    #[get(type(copy))]
    pub(super) page: i32,
    #[get(type(copy))]
    pub(super) limit: u64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogCommentResponse {
    pub(super) id: String,
    pub(super) post_id: String,
    pub(super) user_id: String,
    pub(super) username: String,
    pub(super) avatar: Option<String>,
    pub(super) parent_id: Option<String>,
    pub(super) content: String,
    #[get(type(copy))]
    pub(super) created_at: i64,
    pub(super) replies: Vec<BlogCommentResponse>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogCommentListResponse {
    pub(super) comments: Vec<BlogCommentResponse>,
    #[get(type(copy))]
    pub(super) total: i64,
    #[get(type(copy))]
    pub(super) page: i32,
    #[get(type(copy))]
    pub(super) limit: u64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogImageResponse {
    pub(super) id: String,
    pub(super) post_id: String,
    pub(super) user_id: String,
    pub(super) file_name: String,
    pub(super) original_name: Option<String>,
    pub(super) mime_type: String,
    #[get(type(copy))]
    pub(super) file_size: i32,
    #[get(type(copy))]
    pub(super) created_at: i64,
    pub(super) download_url: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogImageDataResponse {
    pub(super) id: String,
    pub(super) post_id: String,
    pub(super) file_name: String,
    pub(super) original_name: Option<String>,
    pub(super) mime_type: String,
    pub(super) file_data: Vec<u8>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogLikeStatusResponse {
    #[get(type(copy))]
    pub(super) liked: bool,
    #[get(type(copy))]
    pub(super) like_count: i32,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogFavoriteStatusResponse {
    #[get(type(copy))]
    pub(super) favorited: bool,
    #[get(type(copy))]
    pub(super) favorite_count: i32,
}
