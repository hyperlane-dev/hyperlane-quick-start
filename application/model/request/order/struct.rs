use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub(super) username: String,
    pub(super) password: String,
    pub(super) nickname: Option<String>,
    pub(super) email: Option<String>,
    pub(super) phone: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct LoginRequest {
    pub(super) username: String,
    pub(super) password: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub(super) username: String,
    pub(super) password: String,
    pub(super) nickname: Option<String>,
    pub(super) email: Option<String>,
    pub(super) phone: Option<String>,
    pub(super) role: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub(super) nickname: Option<String>,
    pub(super) email: Option<String>,
    pub(super) phone: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct ChangePasswordRequest {
    pub(super) old_password: String,
    pub(super) new_password: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CreateRecordRequest {
    #[get(type(copy), pub)]
    pub(super) amount: Decimal,
    pub(super) category: String,
    pub(super) transaction_type: String,
    pub(super) description: Option<String>,
    pub(super) bill_date: Option<NaiveDate>,
    #[get(type(copy), pub)]
    pub(super) target_user_id: Option<i32>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct RecordQueryRequest {
    #[get(type(copy), pub)]
    pub(super) user_id: Option<i32>,
    pub(super) start_date: Option<NaiveDate>,
    pub(super) end_date: Option<NaiveDate>,
    pub(super) category: Option<String>,
    pub(super) transaction_type: Option<String>,
    #[get(type(copy), pub)]
    pub(super) last_id: Option<i32>,
    #[get(type(copy), pub)]
    pub(super) limit: Option<i32>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct ApproveUserRequest {
    #[get(type(copy), pub)]
    pub(super) approved: bool,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct UserListQueryRequest {
    #[get(pub)]
    #[set(pub)]
    pub(super) keyword: Option<String>,
    #[get(type(copy), pub)]
    #[set(pub)]
    pub(super) last_id: Option<i32>,
    #[get(type(copy), pub)]
    #[set(pub)]
    pub(super) limit: Option<i32>,
}
