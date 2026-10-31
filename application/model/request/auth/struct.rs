use super::*;

/// Request body for user registration, containing credentials and optional contact information.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RegisterRequest {
    /// The desired username for the new account.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) username: String,
    /// The plaintext password (will be hashed before storage).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) password: String,
    /// The optional email address for the account.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) email: Option<String>,
    /// The optional phone number for the account.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) phone: Option<String>,
}

/// Request body for user login, containing username and password credentials.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct LoginRequest {
    /// The username of the account to authenticate.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) username: String,
    /// The password for authentication (will be verified against the stored hash).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) password: String,
}

/// Request body for updating user profile information.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateUserRequest {
    /// The optional new email address.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) email: Option<String>,
    /// The optional new phone number.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) phone: Option<String>,
}

/// Request body for changing the user's password.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChangePasswordRequest {
    /// The current password for verification.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) old_password: String,
    /// The new password to set (will be hashed before storage).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) new_password: String,
}

/// Request body for updating a user's account status (e.g., enabling/disabling).
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UpdateUserStatusRequest {
    /// The desired approval status (true = enabled, false = disabled).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) approved: bool,
}

/// Query parameters for listing users with optional filtering and cursor-based pagination.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserListQueryRequest {
    /// The optional keyword to search by username or email.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) keyword: Option<String>,
    /// The optional last seen ID for cursor-based pagination.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) last_id: Option<String>,
    /// The optional maximum number of users to return per page.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) limit: Option<u64>,
}
