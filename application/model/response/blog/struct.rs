use super::*;

/// Represents a blog post response with content, statistics, and user interaction status.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogPostResponse {
    /// The id.
    pub(super) id: String,
    /// The user id.
    pub(super) user_id: String,
    /// The username.
    pub(super) username: Option<String>,
    /// The title.
    pub(super) title: String,
    /// The summary.
    pub(super) summary: Option<String>,
    /// The content.
    pub(super) content: String,
    /// The cover image.
    pub(super) cover_image: Option<BlogImageResponse>,
    /// The images.
    pub(super) images: Vec<BlogImageResponse>,
    /// The is published.
    #[get(type(copy))]
    pub(super) is_published: bool,
    /// The view count.
    #[get(type(copy))]
    pub(super) view_count: i32,
    /// The like count.
    #[get(type(copy))]
    pub(super) like_count: i32,
    /// The favorite count.
    #[get(type(copy))]
    pub(super) favorite_count: i32,
    /// The comment count.
    #[get(type(copy))]
    pub(super) comment_count: i32,
    /// The is liked.
    #[get(type(copy))]
    pub(super) is_liked: bool,
    /// The is favorited.
    #[get(type(copy))]
    pub(super) is_favorited: bool,
    /// The created at.
    #[get(type(copy))]
    pub(super) created_at: i64,
    /// The updated at.
    #[get(type(copy))]
    pub(super) updated_at: i64,
}

/// blog post list response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogPostListResponse {
    /// The posts.
    pub(super) posts: Vec<BlogPostResponse>,
    /// The total.
    #[get(type(copy))]
    pub(super) total: i64,
    /// The page.
    #[get(type(copy))]
    pub(super) page: i32,
    /// The limit.
    #[get(type(copy))]
    pub(super) limit: u64,
}

/// blog comment response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogCommentResponse {
    /// The id.
    pub(super) id: String,
    /// The post id.
    pub(super) post_id: String,
    /// The user id.
    pub(super) user_id: String,
    /// The username.
    pub(super) username: String,
    /// The avatar.
    pub(super) avatar: Option<String>,
    /// The parent id.
    pub(super) parent_id: Option<String>,
    /// The content.
    pub(super) content: String,
    /// The created at.
    #[get(type(copy))]
    pub(super) created_at: i64,
    /// The replies.
    pub(super) replies: Vec<BlogCommentResponse>,
}

/// blog comment list response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogCommentListResponse {
    /// The comments.
    pub(super) comments: Vec<BlogCommentResponse>,
    /// The total.
    #[get(type(copy))]
    pub(super) total: i64,
    /// The page.
    #[get(type(copy))]
    pub(super) page: i32,
    /// The limit.
    #[get(type(copy))]
    pub(super) limit: u64,
}

/// Represents a blog image response with download URL and metadata.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogImageResponse {
    /// The id.
    pub(super) id: String,
    /// The post id.
    pub(super) post_id: String,
    /// The user id.
    pub(super) user_id: String,
    /// The file name.
    pub(super) file_name: String,
    /// The original name.
    pub(super) original_name: Option<String>,
    /// The mime type.
    pub(super) mime_type: String,
    /// The file size.
    #[get(type(copy))]
    pub(super) file_size: i32,
    /// The created at.
    #[get(type(copy))]
    pub(super) created_at: i64,
    /// The download url.
    pub(super) download_url: String,
}

/// blog image data response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogImageDataResponse {
    /// The id.
    pub(super) id: String,
    /// The post id.
    pub(super) post_id: String,
    /// The file name.
    pub(super) file_name: String,
    /// The original name.
    pub(super) original_name: Option<String>,
    /// The mime type.
    pub(super) mime_type: String,
    /// The file data.
    pub(super) file_data: Vec<u8>,
}

/// blog like status response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogLikeStatusResponse {
    /// The liked.
    #[get(type(copy))]
    pub(super) liked: bool,
    /// The like count.
    #[get(type(copy))]
    pub(super) like_count: i32,
}

/// blog favorite status response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogFavoriteStatusResponse {
    /// The favorited.
    #[get(type(copy))]
    pub(super) favorited: bool,
    /// The favorite count.
    #[get(type(copy))]
    pub(super) favorite_count: i32,
}
