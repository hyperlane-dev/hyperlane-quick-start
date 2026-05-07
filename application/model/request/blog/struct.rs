use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateBlogPostRequest {
    pub(super) title: String,
    pub(super) summary: Option<String>,
    pub(super) content: String,
    pub(super) cover_image_id: Option<String>,
    #[get(type(copy))]
    pub(super) is_published: bool,
    pub(super) image_ids: Vec<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateBlogPostRequest {
    pub(super) title: Option<String>,
    pub(super) summary: Option<String>,
    pub(super) content: Option<String>,
    pub(super) cover_image_id: Option<String>,
    #[get(type(copy))]
    pub(super) is_published: Option<bool>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogPostListQueryRequest {
    pub(super) keyword: Option<String>,
    #[get(type(copy))]
    pub(super) is_published: Option<bool>,
    #[get(type(copy))]
    pub(super) page: Option<i32>,
    #[get(type(copy))]
    pub(super) limit: Option<u64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateBlogCommentRequest {
    pub(super) post_id: String,
    pub(super) parent_id: Option<String>,
    pub(super) content: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogCommentListQueryRequest {
    pub(super) post_id: String,
    #[get(type(copy))]
    pub(super) page: Option<i32>,
    #[get(type(copy))]
    pub(super) limit: Option<u64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct BlogImageUploadRequest {
    pub(super) file_name: String,
    pub(super) original_name: Option<String>,
    pub(super) mime_type: String,
    #[get(type(copy))]
    pub(super) file_size: i32,
    #[schema(value_type = String, format = Binary)]
    pub(super) file_data: Vec<u8>,
}
