use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct RegisterRequest {
    #[get(pub)]
    pub(super) username: String,
    #[get(pub)]
    pub(super) password: String,
    #[get(pub)]
    pub(super) nickname: Option<String>,
    #[get(pub)]
    pub(super) email: Option<String>,
    #[get(pub)]
    pub(super) phone: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct LoginRequest {
    #[get(pub)]
    pub(super) username: String,
    #[get(pub)]
    pub(super) password: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CreateUserRequest {
    #[get(pub)]
    pub(super) username: String,
    #[get(pub)]
    pub(super) password: String,
    #[get(pub)]
    pub(super) nickname: Option<String>,
    #[get(pub)]
    pub(super) email: Option<String>,
    #[get(pub)]
    pub(super) phone: Option<String>,
    #[get(pub)]
    pub(super) role: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    #[get(pub)]
    pub(super) nickname: Option<String>,
    #[get(pub)]
    pub(super) email: Option<String>,
    #[get(pub)]
    pub(super) phone: Option<String>,
    #[get(pub)]
    pub(super) role: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct ChangePasswordRequest {
    #[get(pub)]
    pub(super) old_password: String,
    #[get(pub)]
    pub(super) new_password: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CreateRecordRequest {
    #[get(type(copy), pub)]
    pub(super) amount: Decimal,
    #[get(pub)]
    pub(super) category: String,
    #[get(pub)]
    pub(super) transaction_type: String,
    #[get(pub)]
    pub(super) description: Option<String>,
    #[get(type(copy), pub)]
    pub(super) bill_date: NaiveDate,
    #[get(type(copy), pub)]
    pub(super) target_user_id: Option<i32>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct RecordQueryRequest {
    #[get(type(copy), pub)]
    pub(super) user_id: Option<i32>,
    #[get(pub)]
    pub(super) start_date: Option<NaiveDate>,
    #[get(pub)]
    pub(super) end_date: Option<NaiveDate>,
    #[get(pub)]
    pub(super) category: Option<String>,
    #[get(pub)]
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
