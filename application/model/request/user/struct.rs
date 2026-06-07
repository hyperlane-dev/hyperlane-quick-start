use super::*;

/// Represents a request to update user profile information.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateUserRequest {
    /// The email.
    pub(super) email: Option<String>,
    /// The phone.
    pub(super) phone: Option<String>,
}

/// change password request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChangePasswordRequest {
    /// The old password.
    pub(super) old_password: String,
    /// The new password.
    pub(super) new_password: String,
}

/// update user status request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateUserStatusRequest {
    /// The approved.
    #[get(type(copy))]
    pub(super) approved: bool,
}

/// user list query request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserListQueryRequest {
    /// The keyword.
    pub(super) keyword: Option<String>,
    /// The last id.
    pub(super) last_id: Option<String>,
    /// The limit.
    #[get(type(copy))]
    pub(super) limit: Option<u64>,
}
