use super::*;

/// The profile of a user.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct UserProfile {
    /// The opaque identifier of the user.
    pub id: String,
    /// The name of the user.
    pub username: String,
    /// The address of the user.
    pub email: Option<String>,
    /// The phone number of the user.
    pub phone: Option<String>,
    /// The role of the user.
    pub role: String,
    /// The status of the user.
    pub status: String,
    /// The creation time in milliseconds since the Unix epoch.
    pub created_at: Option<i64>,
}

/// One page of the users of the server.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct UserList {
    /// The users of the page.
    pub users: Vec<UserProfile>,
    /// Whether the server holds more users than the page holds.
    pub has_more: bool,
    /// The identifier the next page starts after.
    pub last_id: Option<String>,
    /// The number of users the server holds.
    pub total_count: i64,
}

/// The contact fields of a user.
#[derive(Clone, Debug, Default, Serialize)]
pub struct UpdateUserRequest {
    /// The new address of the user.
    pub email: Option<String>,
    /// The new phone number of the user.
    pub phone: Option<String>,
}

/// The passwords of a user.
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangePasswordRequest {
    /// The current password of the user.
    pub old_password: String,
    /// The new password of the user.
    pub new_password: String,
}

/// The status a user is set to.
#[derive(Clone, Debug, Default, Serialize)]
pub struct UpdateStatusRequest {
    /// Whether the user is approved.
    pub approved: bool,
}
