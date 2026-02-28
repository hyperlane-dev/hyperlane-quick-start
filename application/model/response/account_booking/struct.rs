use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct UserResponse {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(pub)]
    pub(super) username: String,
    #[get(pub)]
    pub(super) nickname: Option<String>,
    #[get(pub)]
    pub(super) email: Option<String>,
    #[get(pub)]
    pub(super) phone: Option<String>,
    #[get(pub)]
    pub(super) role: String,
    #[get(pub)]
    pub(super) status: String,
    #[get(pub)]
    pub(super) created_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct LoginResponse {
    #[get(pub)]
    pub(super) user: UserResponse,
    #[get(pub)]
    pub(super) token: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct RecordResponse {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(pub)]
    pub(super) bill_no: String,
    #[get(type(copy), pub)]
    pub(super) user_id: i32,
    #[get(pub)]
    pub(super) amount: String,
    #[get(pub)]
    pub(super) category: String,
    #[get(pub)]
    pub(super) transaction_type: String,
    #[get(pub)]
    pub(super) description: Option<String>,
    #[get(pub)]
    pub(super) bill_date: String,
    #[get(pub)]
    pub(super) created_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct RecordListResponse {
    #[get(pub)]
    pub(super) records: Vec<RecordResponse>,
    #[get(pub)]
    pub(super) total_income: String,
    #[get(pub)]
    pub(super) total_expense: String,
    #[get(pub)]
    pub(super) balance: String,
}
