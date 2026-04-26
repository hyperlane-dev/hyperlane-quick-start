use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RsaPublicKeyResponse {
    pub(super) n: String,
    pub(super) e: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserResponse {
    pub(super) id: String,
    pub(super) username: String,
    pub(super) email: Option<String>,
    pub(super) phone: Option<String>,
    pub(super) role: String,
    pub(super) status: String,
    pub(super) created_at: Option<i64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct LoginResponse {
    pub(super) user: UserResponse,
    pub(super) token: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserListResponse {
    pub(super) users: Vec<UserResponse>,
    #[get(type(copy))]
    pub(super) has_more: bool,
    pub(super) last_id: Option<String>,
    #[get(type(copy))]
    pub(super) total_count: i64,
}
