use super::*;

/// create blog post request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateBlogPostRequest {
    /// The title.
    pub(super) title: String,
    /// The summary.
    pub(super) summary: Option<String>,
    /// The content.
    pub(super) content: String,
    /// The cover image id.
    pub(super) cover_image_id: Option<String>,
    /// The is published.
    #[get(type(copy))]
    pub(super) is_published: bool,
    /// The image ids.
    pub(super) image_ids: Vec<String>,
}

/// update blog post request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateBlogPostRequest {
    /// The title.
    pub(super) title: Option<String>,
    /// The summary.
    pub(super) summary: Option<String>,
    /// The content.
    pub(super) content: Option<String>,
    /// The cover image id.
    pub(super) cover_image_id: Option<String>,
    /// The is published.
    #[get(type(copy))]
    pub(super) is_published: Option<bool>,
}

/// blog post list query request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogPostListQueryRequest {
    /// The keyword.
    pub(super) keyword: Option<String>,
    /// The is published.
    #[get(type(copy))]
    pub(super) is_published: Option<bool>,
    /// The page.
    #[get(type(copy))]
    pub(super) page: Option<i32>,
    /// The limit.
    #[get(type(copy))]
    pub(super) limit: Option<u64>,
}

/// create blog comment request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateBlogCommentRequest {
    /// The post id.
    pub(super) post_id: String,
    /// The parent id.
    pub(super) parent_id: Option<String>,
    /// The content.
    pub(super) content: String,
}

/// blog comment list query request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogCommentListQueryRequest {
    /// The post id.
    pub(super) post_id: String,
    /// The page.
    #[get(type(copy))]
    pub(super) page: Option<i32>,
    /// The limit.
    #[get(type(copy))]
    pub(super) limit: Option<u64>,
}

/// blog image upload request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogImageUploadRequest {
    /// The file name.
    pub(super) file_name: String,
    /// The original name.
    pub(super) original_name: Option<String>,
    /// The mime type.
    pub(super) mime_type: String,
    /// The file size.
    #[get(type(copy))]
    pub(super) file_size: i32,
    /// The file data.
    #[schema(value_type = String, format = Binary)]
    pub(super) file_data: Vec<u8>,
}
