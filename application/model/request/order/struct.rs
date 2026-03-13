use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RegisterRequest {
    pub(super) username: String,
    pub(super) password: String,
    pub(super) email: Option<String>,
    pub(super) phone: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct LoginRequest {
    pub(super) username: String,
    pub(super) password: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateUserRequest {
    pub(super) email: Option<String>,
    pub(super) phone: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChangePasswordRequest {
    pub(super) old_password: String,
    pub(super) new_password: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateRecordRequest {
    #[get(type(copy), pub)]
    #[set(pub)]
    #[schema(value_type = String)]
    pub(super) amount: Decimal,
    #[set(pub)]
    pub(super) category: String,
    #[set(pub)]
    pub(super) transaction_type: String,
    #[set(pub)]
    pub(super) description: Option<String>,
    #[schema(value_type = String)]
    #[set(pub)]
    pub(super) bill_date: Option<NaiveDate>,
    #[get(type(copy), pub)]
    #[set(pub)]
    pub(super) target_user_id: Option<i32>,
    #[set(pub)]
    pub(super) images: Vec<ImageUploadRequest>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RecordQueryRequest {
    #[get(type(copy), pub)]
    pub(super) user_id: Option<i32>,
    #[schema(value_type = String)]
    pub(super) start_date: Option<NaiveDate>,
    #[schema(value_type = String)]
    pub(super) end_date: Option<NaiveDate>,
    pub(super) category: Option<String>,
    pub(super) transaction_type: Option<String>,
    #[get(type(copy), pub)]
    pub(super) cache_id: Option<i32>,
    #[get(type(copy), pub)]
    pub(super) page: Option<i32>,
    #[get(type(copy), pub)]
    pub(super) limit: Option<u64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ApproveUserRequest {
    #[get(type(copy), pub)]
    pub(super) approved: bool,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserListQueryRequest {
    #[set(pub)]
    pub(super) keyword: Option<String>,
    #[get(type(copy), pub)]
    #[set(pub)]
    pub(super) last_id: Option<i32>,
    #[get(type(copy), pub)]
    #[set(pub)]
    pub(super) limit: Option<u64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateRecordWithImagesRequest {
    #[get(type(copy), pub)]
    #[schema(value_type = String)]
    pub(super) amount: Decimal,
    pub(super) category: String,
    pub(super) transaction_type: String,
    pub(super) description: Option<String>,
    #[schema(value_type = String)]
    pub(super) bill_date: Option<NaiveDate>,
    #[get(type(copy), pub)]
    pub(super) target_user_id: Option<i32>,
    pub(super) images: Vec<ImageUploadRequest>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ImageUploadRequest {
    #[set(pub)]
    pub(super) file_name: String,
    #[set(pub)]
    pub(super) original_name: Option<String>,
    #[set(pub)]
    pub(super) mime_type: String,
    #[get(type(copy), pub)]
    #[set(pub)]
    pub(super) file_size: i32,
    #[schema(value_type = String, format = Binary)]
    #[set(pub)]
    pub(super) file_data: Vec<u8>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RecordImageQueryRequest {
    #[get(type(copy), pub)]
    pub(super) record_id: i32,
}

#[derive(Clone, Data, Debug, Default)]
pub struct RecordPaginationQuery {
    #[get(type(copy), pub)]
    pub(super) user_id: Option<i32>,
    #[get(type(copy), pub)]
    pub(super) start_date: Option<NaiveDate>,
    #[get(type(copy), pub)]
    pub(super) end_date: Option<NaiveDate>,
    #[get(type(option), pub)]
    pub(super) category: Option<String>,
    #[get(type(option), pub)]
    pub(super) transaction_type: Option<String>,
    #[get(type(copy), pub)]
    pub(super) cache_id: Option<i32>,
    #[get(type(copy), pub)]
    pub(super) page: i32,
    #[get(type(copy), pub)]
    pub(super) limit: u64,
}
