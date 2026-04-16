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
pub struct ApproveUserRequest {
    #[get(type(copy))]
    pub(super) approved: bool,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserListQueryRequest {
    pub(super) keyword: Option<String>,
    pub(super) last_id: Option<String>,
    #[get(type(copy))]
    pub(super) limit: Option<u64>,
}
