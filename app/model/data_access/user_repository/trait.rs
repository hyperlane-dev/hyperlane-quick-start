use super::*;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, UserRepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserRepositoryError>;
    async fn create_user(&self, request: CreateUserRequest) -> Result<User, UserRepositoryError>;
    async fn update_user(
        &self,
        id: i64,
        request: UpdateUserRequest,
    ) -> Result<User, UserRepositoryError>;
    async fn delete_user(&self, id: i64) -> Result<bool, UserRepositoryError>;
    async fn verify_password(
        &self,
        username: &str,
        password: &str,
    ) -> Result<bool, UserRepositoryError>;
    async fn change_password(
        &self,
        id: i64,
        new_password_hash: &str,
    ) -> Result<(), UserRepositoryError>;
    async fn list_users(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<User>, UserRepositoryError>;
    async fn count_users(&self) -> Result<i64, UserRepositoryError>;
}

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Pool error: {0}")]
    PoolError(#[from] hyperlane_config::framework::ConnectionPoolError),

    #[error("Validation error: {0}")]
    ValidationError(#[from] UserValidationError),

    #[error("Username already exists")]
    UsernameAlreadyExists,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Password hashing error: {0}")]
    PasswordHashingError(String),

    #[error("Password verification error: {0}")]
    PasswordVerificationError(String),

    #[error("No fields to update")]
    NoFieldsToUpdate,
}
