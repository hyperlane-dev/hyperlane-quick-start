use super::*;

impl AuthService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { 
            user_repository,
            session_manager: Arc::new(SessionManager::default()),
        }
    }

    pub fn new_with_session_timeout(
        user_repository: Arc<dyn UserRepository>, 
        session_timeout_hours: i64
    ) -> Self {
        Self { 
            user_repository,
            session_manager: Arc::new(SessionManager::new(session_timeout_hours)),
        }
    }

    pub fn from_global_pool() -> Result<Self, AuthError> {
        let user_repository = Arc::new(
            PostgresUserRepository::from_global_pool().map_err(AuthError::RepositoryError)?,
        );
        Ok(Self::new(user_repository))
    }

    pub fn from_global_pool_with_session_timeout(session_timeout_hours: i64) -> Result<Self, AuthError> {
        let user_repository = Arc::new(
            PostgresUserRepository::from_global_pool().map_err(AuthError::RepositoryError)?,
        );
        Ok(Self::new_with_session_timeout(user_repository, session_timeout_hours))
    }

    /// Authenticate user with username and password
    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, AuthError> {
        // Validate request
        request.validate().map_err(AuthError::ValidationError)?;

        // Find user by username
        let user = self
            .user_repository
            .find_by_username(&request.username)
            .await
            .map_err(AuthError::RepositoryError)?;

        match user {
            Some(user) => {
                if !user.is_active {
                    return Ok(LoginResponse::failure("Account is disabled".to_string()));
                }

                // Verify password
                let password_valid =
                    PasswordService::verify_password(&request.password, &user.password_hash)
                        .map_err(AuthError::PasswordError)?;

                if password_valid {
                    // Create session
                    let session_info = self.session_manager
                        .create_session(user.id, user.username.clone())
                        .map_err(AuthError::SessionError)?;
                    
                    let mut response = LoginResponse::success(user.id, user.username);
                    response.session_id = Some(session_info.session_id);
                    response.expires_at = Some(session_info.expires_at);
                    Ok(response)
                } else {
                    Ok(LoginResponse::failure("Invalid credentials".to_string()))
                }
            }
            None => {
                // To prevent username enumeration, we still perform a dummy password check
                let _ = PasswordService::verify_password(
                    &request.password,
                    "$2b$12$dummy.hash.to.prevent.timing.attacks",
                );
                Ok(LoginResponse::failure("Invalid credentials".to_string()))
            }
        }
    }

    /// Register a new user
    pub async fn register(&self, request: CreateUserRequest) -> Result<UserResponse, AuthError> {
        // Validate request
        request.validate().map_err(AuthError::ValidationError)?;

        // Check password strength
        let password_strength = PasswordService::check_password_strength(&request.password);
        if matches!(password_strength.level, PasswordStrengthLevel::Weak) {
            return Err(AuthError::WeakPassword(password_strength.feedback));
        }

        // Create user through repository (which handles password hashing)
        let user = self
            .user_repository
            .create_user(request)
            .await
            .map_err(AuthError::RepositoryError)?;

        Ok(user.sanitize_for_response())
    }

    /// Change user password
    pub async fn change_password(
        &self,
        user_id: i64,
        request: ChangePasswordRequest,
    ) -> Result<(), AuthError> {
        // Validate request
        request.validate().map_err(AuthError::ValidationError)?;

        // Find user
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await
            .map_err(AuthError::RepositoryError)?
            .ok_or(AuthError::UserNotFound)?;

        // Verify current password
        let current_password_valid =
            PasswordService::verify_password(&request.current_password, &user.password_hash)
                .map_err(AuthError::PasswordError)?;

        if !current_password_valid {
            return Err(AuthError::InvalidCurrentPassword);
        }

        // Check new password strength
        let password_strength = PasswordService::check_password_strength(&request.new_password);
        if matches!(password_strength.level, PasswordStrengthLevel::Weak) {
            return Err(AuthError::WeakPassword(password_strength.feedback));
        }

        // Hash new password
        let new_password_hash = PasswordService::hash_password(&request.new_password)
            .map_err(AuthError::PasswordError)?;

        // Update password in repository
        self.user_repository
            .change_password(user_id, &new_password_hash)
            .await
            .map_err(AuthError::RepositoryError)?;

        Ok(())
    }

    /// Get user profile
    pub async fn get_user_profile(&self, user_id: i64) -> Result<UserResponse, AuthError> {
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await
            .map_err(AuthError::RepositoryError)?
            .ok_or(AuthError::UserNotFound)?;

        Ok(user.sanitize_for_response())
    }

    /// Update user profile
    pub async fn update_user_profile(
        &self,
        user_id: i64,
        request: UpdateUserRequest,
    ) -> Result<UserResponse, AuthError> {
        let user = self
            .user_repository
            .update_user(user_id, request)
            .await
            .map_err(AuthError::RepositoryError)?;

        Ok(user.sanitize_for_response())
    }

    /// Check if username is available
    pub async fn is_username_available(&self, username: &str) -> Result<bool, AuthError> {
        if !User::is_valid_username(username) {
            return Ok(false);
        }

        let user = self
            .user_repository
            .find_by_username(username)
            .await
            .map_err(AuthError::RepositoryError)?;

        Ok(user.is_none())
    }

    /// Check if email is available
    pub async fn is_email_available(&self, email: &str) -> Result<bool, AuthError> {
        if !User::is_valid_email(email) {
            return Ok(false);
        }

        let user = self
            .user_repository
            .find_by_email(email)
            .await
            .map_err(AuthError::RepositoryError)?;

        Ok(user.is_none())
    }

    /// Generate a secure password
    pub fn generate_secure_password(&self, length: Option<usize>) -> Result<String, AuthError> {
        let length = length.unwrap_or(16);
        PasswordService::generate_password(length).map_err(AuthError::PasswordError)
    }

    /// Check password strength
    pub fn check_password_strength(&self, password: &str) -> PasswordStrength {
        PasswordService::check_password_strength(password)
    }

    /// List users (admin function)
    pub async fn list_users(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<UserResponse>, AuthError> {
        let users = self
            .user_repository
            .list_users(limit, offset)
            .await
            .map_err(AuthError::RepositoryError)?;

        Ok(users
            .into_iter()
            .map(|user| user.sanitize_for_response())
            .collect())
    }

    /// Get user count (admin function)
    pub async fn get_user_count(&self) -> Result<i64, AuthError> {
        self.user_repository
            .count_users()
            .await
            .map_err(AuthError::RepositoryError)
    }

    /// Delete user (admin function)
    pub async fn delete_user(&self, user_id: i64) -> Result<bool, AuthError> {
        self.user_repository
            .delete_user(user_id)
            .await
            .map_err(AuthError::RepositoryError)
    }

    /// Logout user (invalidate session)
    pub fn logout(&self, session_id: &str) -> Result<(), AuthError> {
        self.session_manager
            .invalidate_session(session_id)
            .map_err(AuthError::SessionError)
    }

    /// Validate session and get user info
    pub fn validate_session(&self, session_id: &str) -> Result<SessionInfo, AuthError> {
        self.session_manager
            .validate_session(session_id)
            .map_err(AuthError::SessionError)
    }

    /// Logout all sessions for a user
    pub fn logout_all_sessions(&self, user_id: i64) -> Result<usize, AuthError> {
        self.session_manager
            .invalidate_user_sessions(user_id)
            .map_err(AuthError::SessionError)
    }

    /// Get all active sessions for a user
    pub fn get_user_sessions(&self, user_id: i64) -> Result<Vec<SessionInfo>, AuthError> {
        self.session_manager
            .get_user_sessions(user_id)
            .map_err(AuthError::SessionError)
    }

    /// Clean up expired sessions
    pub fn cleanup_expired_sessions(&self) -> Result<usize, AuthError> {
        self.session_manager
            .cleanup_expired_sessions()
            .map_err(AuthError::SessionError)
    }

    /// Get session statistics (admin function)
    pub fn get_session_stats(&self) -> Result<SessionStats, AuthError> {
        self.session_manager
            .get_session_stats()
            .map_err(AuthError::SessionError)
    }
}

impl AuthError {
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            AuthError::ValidationError(_)
                | AuthError::UserNotFound
                | AuthError::InvalidCurrentPassword
                | AuthError::WeakPassword(_)
                | AuthError::AccountDisabled
                | AuthError::UsernameAlreadyExists
                | AuthError::EmailAlreadyExists
                | AuthError::SessionError(SessionError::SessionNotFound)
                | AuthError::SessionError(SessionError::SessionExpired)
                | AuthError::SessionError(SessionError::SessionInactive)
                | AuthError::SessionError(SessionError::InvalidSessionId)
        )
    }

    pub fn to_user_message(&self) -> String {
        match self {
            AuthError::ValidationError(e) => e.to_string(),
            AuthError::UserNotFound => "User not found".to_string(),
            AuthError::InvalidCurrentPassword => "Current password is incorrect".to_string(),
            AuthError::WeakPassword(feedback) => {
                format!("Password is too weak: {}", feedback.join(", "))
            }
            AuthError::AccountDisabled => "Account is disabled".to_string(),
            AuthError::UsernameAlreadyExists => "Username already exists".to_string(),
            AuthError::EmailAlreadyExists => "Email already exists".to_string(),
            AuthError::SessionError(SessionError::SessionNotFound) => "Session not found".to_string(),
            AuthError::SessionError(SessionError::SessionExpired) => "Session expired".to_string(),
            AuthError::SessionError(SessionError::SessionInactive) => "Session inactive".to_string(),
            AuthError::SessionError(SessionError::InvalidSessionId) => "Invalid session".to_string(),
            _ => "An internal error occurred".to_string(),
        }
    }
}
