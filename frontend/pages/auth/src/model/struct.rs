use super::*;

/// The public key the credentials are encrypted with.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct RsaPublicKey {
    /// The base64 encoded modulus of the key.
    pub modulus: String,
    /// The base64 encoded public exponent of the key.
    pub exponent: String,
}

/// The credentials of a login.
#[derive(Clone, Debug, Default, Serialize)]
pub struct LoginRequest {
    /// The encrypted name of the account.
    pub username: String,
    /// The encrypted password of the account.
    pub password: String,
}

/// The credentials of a registration.
#[derive(Clone, Debug, Default, Serialize)]
pub struct RegisterRequest {
    /// The encrypted name of the account.
    pub username: String,
    /// The encrypted password of the account.
    pub password: String,
    /// The encrypted address of the account.
    pub email: Option<String>,
    /// The encrypted phone number of the account.
    pub phone: Option<String>,
}

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

/// The answer of a successful login.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct LoginResponse {
    /// The profile of the user that logged in.
    pub user: UserProfile,
    /// The token of the session.
    pub token: String,
}
