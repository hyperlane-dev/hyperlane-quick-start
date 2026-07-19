/// Rsa key cache ttl secs.
pub const RSA_KEY_CACHE_TTL_SECS: u64 = 3600;

/// Error message when rsa private key not initialized.
pub const ERROR_RSA_PRIVATE_KEY_NOT_INITIALIZED: &str = "RSA private key not initialized";

/// Error message when failed to encode id.
pub const ERROR_FAILED_TO_ENCODE_ID: &str = "Failed to encode ID";

/// Error message when invalid id format.
pub const ERROR_INVALID_ID_FORMAT: &str = "Invalid ID format";

/// Error message when authentication token not found.
pub const ERROR_AUTHENTICATION_TOKEN_NOT_FOUND: &str = "Authentication token not found";

/// Error message when user id not found in token.
pub const ERROR_USER_ID_NOT_FOUND_IN_TOKEN: &str = "user_id not found in token";

/// Error message when invalid token.
pub const ERROR_INVALID_TOKEN: &str = "Invalid token";

/// Error message when invalid user id format in token.
pub const ERROR_INVALID_USER_ID_FORMAT_IN_TOKEN: &str = "Invalid user_id format in token";

/// Error message when invalid email format.
pub const ERROR_INVALID_EMAIL_FORMAT: &str = "Invalid email format";

/// Error message when invalid phone format.
pub const ERROR_INVALID_PHONE_FORMAT: &str = "Invalid phone format";

/// Error message when username already exists.
pub const ERROR_USERNAME_ALREADY_EXISTS: &str = "Username already exists";

/// Error message when user is not approved.
pub const ERROR_USER_IS_NOT_APPROVED: &str = "User is not approved";

/// Error message when invalid password.
pub const ERROR_INVALID_PASSWORD: &str = "Invalid password";

/// Error message when user not found.
pub const ERROR_USER_NOT_FOUND: &str = "User not found";

/// Error message when old password is incorrect.
pub const ERROR_OLD_PASSWORD_IS_INCORRECT: &str = "Old password is incorrect";

/// JWT claim user id.
pub const JWT_CLAIM_USER_ID: &str = "user_id";

/// JWT claim role.
pub const JWT_CLAIM_ROLE: &str = "role";

/// Default page limit.
pub const DEFAULT_PAGE_LIMIT: u64 = 20;

/// JWT expiration seconds.
pub const JWT_EXPIRATION_SECONDS: u64 = 86400;

/// Cookie format for format.
pub const COOKIE_FORMAT: &str =
    "token={token_str}; Path=/; Max-Age={JWT_EXPIRATION_SECONDS}; HttpOnly";

/// Cookie format for format.
pub const CLEAR_COOKIE_FORMAT: &str = "token=; Path=/; Max-Age=0; HttpOnly";
