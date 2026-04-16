use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateRecordRequest {
    #[get(type(copy))]
    #[schema(value_type = String)]
    pub(super) amount: Decimal,
    pub(super) category: String,
    pub(super) transaction_type: String,
    pub(super) description: Option<String>,
    #[schema(value_type = String)]
    pub(super) bill_date: Option<NaiveDate>,
    pub(super) target_user_id: Option<String>,
    pub(super) image_ids: Vec<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RecordQueryRequest {
    #[get(type(copy))]
    pub(super) user_id: Option<i32>,
    #[schema(value_type = String)]
    pub(super) start_date: Option<NaiveDate>,
    #[schema(value_type = String)]
    pub(super) end_date: Option<NaiveDate>,
    pub(super) category: Option<String>,
    pub(super) transaction_type: Option<String>,
    #[get(type(copy))]
    pub(super) cache_id: Option<i32>,
    #[get(type(copy))]
    pub(super) page: Option<i32>,
    #[get(type(copy))]
    pub(super) limit: Option<u64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateRecordWithImagesRequest {
    #[get(type(copy))]
    #[schema(value_type = String)]
    pub(super) amount: Decimal,
    pub(super) category: String,
    pub(super) transaction_type: String,
    pub(super) description: Option<String>,
    #[schema(value_type = String)]
    pub(super) bill_date: Option<NaiveDate>,
    pub(super) target_user_id: Option<String>,
    pub(super) images: Vec<ImageUploadRequest>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ImageUploadRequest {
    pub(super) file_name: String,
    pub(super) original_name: Option<String>,
    pub(super) mime_type: String,
    #[get(type(copy))]
    pub(super) file_size: i32,
    #[schema(value_type = String, format = Binary)]
    pub(super) file_data: Vec<u8>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RecordImageQueryRequest {
    pub(super) record_id: String,
}

#[derive(Clone, Data, Debug, Default)]
pub struct RecordPaginationQuery {
    #[get(type(copy))]
    pub(super) user_id: Option<i32>,
    #[get(type(copy))]
    pub(super) start_date: Option<NaiveDate>,
    #[get(type(copy))]
    pub(super) end_date: Option<NaiveDate>,
    #[get(type(option))]
    pub(super) category: Option<String>,
    #[get(type(option))]
    pub(super) transaction_type: Option<String>,
    #[get(type(copy))]
    pub(super) cache_id: Option<i32>,
    #[get(type(copy))]
    pub(super) page: i32,
    #[get(type(copy))]
    pub(super) limit: u64,
}
