use super::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Repository error: {0}")]
    RepositoryError(#[from] UserRepositoryError),

    #[error("Password error: {0}")]
    PasswordError(#[from] PasswordError),

    #[error("Validation error: {0}")]
    ValidationError(#[from] UserValidationError),

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid current password")]
    InvalidCurrentPassword,

    #[error("Password is too weak: {0:?}")]
    WeakPassword(Vec<String>),

    #[error("Account is disabled")]
    AccountDisabled,

    #[error("Username already exists")]
    UsernameAlreadyExists,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Session error: {0}")]
    SessionError(#[from] SessionError),
}
