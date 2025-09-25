use super::*;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Password cannot be empty")]
    EmptyPassword,

    #[error("Hash cannot be empty")]
    EmptyHash,

    #[error("Password is too long (max 72 characters for bcrypt)")]
    PasswordTooLong,

    #[error("Invalid password length")]
    InvalidLength,

    #[error("Password hashing failed: {0}")]
    HashingFailed(String),

    #[error("Password verification failed: {0}")]
    VerificationFailed(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PasswordStrengthLevel {
    Weak,
    Medium,
    Strong,
    VeryStrong,
}
