use super::*;

/// create record request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateRecordRequest {
    /// The amount.
    #[get(type(copy))]
    #[schema(value_type = String)]
    pub(super) amount: Decimal,
    /// The category.
    pub(super) category: String,
    /// The transaction type.
    pub(super) transaction_type: String,
    /// The description.
    pub(super) description: Option<String>,
    /// The bill date.
    #[schema(value_type = String)]
    pub(super) bill_date: Option<NaiveDate>,
    /// The target user id.
    pub(super) target_user_id: Option<String>,
    /// The image ids.
    pub(super) image_ids: Vec<String>,
}

/// record query request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RecordQueryRequest {
    /// The user id.
    #[get(type(copy))]
    pub(super) user_id: Option<i32>,
    /// The start date.
    #[schema(value_type = String)]
    pub(super) start_date: Option<NaiveDate>,
    /// The end date.
    #[schema(value_type = String)]
    pub(super) end_date: Option<NaiveDate>,
    /// The category.
    pub(super) category: Option<String>,
    /// The transaction type.
    pub(super) transaction_type: Option<String>,
    /// The cache id.
    #[get(type(copy))]
    pub(super) cache_id: Option<i32>,
    /// The page.
    #[get(type(copy))]
    pub(super) page: Option<i32>,
    /// The limit.
    #[get(type(copy))]
    pub(super) limit: Option<u64>,
}

/// create record with images request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateRecordWithImagesRequest {
    /// The amount.
    #[get(type(copy))]
    #[schema(value_type = String)]
    pub(super) amount: Decimal,
    /// The category.
    pub(super) category: String,
    /// The transaction type.
    pub(super) transaction_type: String,
    /// The description.
    pub(super) description: Option<String>,
    /// The bill date.
    #[schema(value_type = String)]
    pub(super) bill_date: Option<NaiveDate>,
    /// The target user id.
    pub(super) target_user_id: Option<String>,
    /// The images.
    pub(super) images: Vec<ImageUploadRequest>,
}

/// image upload request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ImageUploadRequest {
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

/// record image query request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RecordImageQueryRequest {
    /// The record id.
    pub(super) record_id: String,
}

/// record pagination query.
#[derive(Clone, Data, Debug, Default)]
pub struct RecordPaginationQuery {
    /// The user id.
    #[get(type(copy))]
    pub(super) user_id: Option<i32>,
    /// The start date.
    #[get(type(copy))]
    pub(super) start_date: Option<NaiveDate>,
    /// The end date.
    #[get(type(copy))]
    pub(super) end_date: Option<NaiveDate>,
    /// The category.
    #[get(type(option))]
    pub(super) category: Option<String>,
    /// The transaction type.
    #[get(type(option))]
    pub(super) transaction_type: Option<String>,
    /// The cache id.
    #[get(type(copy))]
    pub(super) cache_id: Option<i32>,
    /// The page.
    #[get(type(copy))]
    pub(super) page: i32,
    /// The limit.
    #[get(type(copy))]
    pub(super) limit: u64,
}
