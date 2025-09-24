use super::*;

impl DatabaseException {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            DatabaseException::ConnectionFailed(_)
                | DatabaseException::PoolExhausted
                | DatabaseException::Timeout(_)
        )
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            DatabaseException::ConnectionFailed(_) => "DB_CONNECTION_FAILED",
            DatabaseException::QueryFailed(_) => "DB_QUERY_FAILED",
            DatabaseException::TransactionFailed(_) => "DB_TRANSACTION_FAILED",
            DatabaseException::PoolExhausted => "DB_POOL_EXHAUSTED",
            DatabaseException::Timeout(_) => "DB_TIMEOUT",
            DatabaseException::IntegrityViolation(_) => "DB_INTEGRITY_VIOLATION",
            DatabaseException::SerializationFailed(_) => "DB_SERIALIZATION_FAILED",
        }
    }
}

impl AuthException {
    pub fn error_code(&self) -> &'static str {
        match self {
            AuthException::AuthenticationFailed => "AUTH_FAILED",
            AuthException::AuthorizationFailed => "AUTHZ_FAILED",
            AuthException::InvalidToken => "INVALID_TOKEN",
            AuthException::TokenExpired => "TOKEN_EXPIRED",
            AuthException::AccountLocked => "ACCOUNT_LOCKED",
            AuthException::PasswordPolicyViolation(_) => "PASSWORD_POLICY_VIOLATION",
        }
    }
}
