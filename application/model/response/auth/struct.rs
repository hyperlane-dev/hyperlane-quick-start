use super::*;

/// rsa public key response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RsaPublicKeyResponse {
    /// The modulus.
    pub(super) modulus: String,
    /// The exponent.
    pub(super) exponent: String,
}

/// Represents an authenticated user profile response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserResponse {
    /// The id.
    pub(super) id: String,
    /// The username.
    pub(super) username: String,
    /// The email.
    pub(super) email: Option<String>,
    /// The phone.
    pub(super) phone: Option<String>,
    /// The role.
    pub(super) role: String,
    /// The status.
    pub(super) status: String,
    /// The created at.
    pub(super) created_at: Option<i64>,
}

/// Represents a login response with user profile and authentication token.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct LoginResponse {
    /// The user.
    pub(super) user: UserResponse,
    /// The token.
    pub(super) token: String,
}

/// Represents a paginated list of authenticated user responses.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserListResponse {
    /// The users.
    pub(super) users: Vec<UserResponse>,
    /// The has more.
    #[get(type(copy))]
    pub(super) has_more: bool,
    /// The last id.
    pub(super) last_id: Option<String>,
    /// The total count.
    #[get(type(copy))]
    pub(super) total_count: i64,
}
