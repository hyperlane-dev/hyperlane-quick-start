use super::*;

/// create record request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateRecordRequest {
    /// The amount.
    #[get(pub(crate), type(copy))]
    #[schema(value_type = String)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) amount: Decimal,
    /// The category.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) category: String,
    /// The transaction type.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) transaction_type: String,
    /// The description.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) description: Option<String>,
    /// The bill date.
    #[schema(value_type = String)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) bill_date: Option<NaiveDate>,
    /// The target user id.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) target_user_id: Option<String>,
    /// The image ids.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) image_ids: Vec<String>,
}

/// record query request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RecordQueryRequest {
    /// The user id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) user_id: Option<i32>,
    /// The start date.
    #[schema(value_type = String)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) start_date: Option<NaiveDate>,
    /// The end date.
    #[schema(value_type = String)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) end_date: Option<NaiveDate>,
    /// The category.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) category: Option<String>,
    /// The transaction type.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) transaction_type: Option<String>,
    /// The cache id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cache_id: Option<i32>,
    /// The page.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page: Option<i32>,
    /// The limit.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) limit: Option<u64>,
}

/// create record with images request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateRecordWithImagesRequest {
    /// The amount.
    #[get(pub(crate), type(copy))]
    #[schema(value_type = String)]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) amount: Decimal,
    /// The category.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) category: String,
    /// The transaction type.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) transaction_type: String,
    /// The description.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) description: Option<String>,
    /// The bill date.
    #[schema(value_type = String)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) bill_date: Option<NaiveDate>,
    /// The target user id.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) target_user_id: Option<String>,
    /// The images.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) images: Vec<ImageUploadRequest>,
}

/// image upload request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ImageUploadRequest {
    /// The file name.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) file_name: String,
    /// The original name.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) original_name: Option<String>,
    /// The mime type.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) mime_type: String,
    /// The file size.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) file_size: i32,
    /// The file data.
    #[schema(value_type = String, format = Binary)]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) file_data: Vec<u8>,
}

/// record image query request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RecordImageQueryRequest {
    /// The record id.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) record_id: String,
}

/// record pagination query.
#[derive(Clone, Data, Debug, Default)]
pub struct RecordPaginationQuery {
    /// The user id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) user_id: Option<i32>,
    /// The start date.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) start_date: Option<NaiveDate>,
    /// The end date.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) end_date: Option<NaiveDate>,
    /// The category.
    #[get(pub(crate), type(option))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) category: Option<String>,
    /// The transaction type.
    #[get(pub(crate), type(option))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) transaction_type: Option<String>,
    /// The cache id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cache_id: Option<i32>,
    /// The page.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page: i32,
    /// The limit.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) limit: u64,
}
