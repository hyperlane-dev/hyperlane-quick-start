use super::*;

/// Represents a user profile response with encoded ID and role information.
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

/// Represents a paginated list of user responses with cursor-based pagination.
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
