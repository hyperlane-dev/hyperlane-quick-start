use super::*;

#[derive(Debug, Error)]
pub enum AuthException {
    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Authorization failed")]
    AuthorizationFailed,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("Account locked")]
    AccountLocked,

    #[error("Password policy violation: {0}")]
    PasswordPolicyViolation(String),
}
