use super::*;

impl PasswordService {
    /// Hash a password using bcrypt with default cost
    pub fn hash_password(password: &str) -> Result<String, PasswordError> {
        Self::hash_password_with_cost(password, DEFAULT_COST)
    }

    /// Hash a password using bcrypt with custom cost
    pub fn hash_password_with_cost(password: &str, cost: u32) -> Result<String, PasswordError> {
        if password.is_empty() {
            return Err(PasswordError::EmptyPassword);
        }

        if password.len() > 72 {
            return Err(PasswordError::PasswordTooLong);
        }

        let start_time = Instant::now();
        let hash_result =
            hash(password, cost).map_err(|e| PasswordError::HashingFailed(e.to_string()))?;

        let duration = start_time.elapsed();
        if duration.as_millis() > 1000 {
            eprintln!(
                "Warning: Password hashing took {}ms, consider reducing cost",
                duration.as_millis()
            );
        }

        Ok(hash_result)
    }

    /// Verify a password against its hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, PasswordError> {
        if password.is_empty() {
            return Err(PasswordError::EmptyPassword);
        }

        if hash.is_empty() {
            return Err(PasswordError::EmptyHash);
        }

        let start_time = Instant::now();
        let verify_result =
            verify(password, hash).map_err(|e| PasswordError::VerificationFailed(e.to_string()))?;

        let duration = start_time.elapsed();
        if duration.as_millis() > 500 {
            eprintln!(
                "Warning: Password verification took {}ms",
                duration.as_millis()
            );
        }

        Ok(verify_result)
    }

    /// Check password strength
    pub fn check_password_strength(password: &str) -> PasswordStrength {
        let mut score: u8 = 0;
        let mut feedback = Vec::new();

        // Length check
        if password.len() >= 8 {
            score += 1;
        } else {
            feedback.push("Password should be at least 8 characters long".to_string());
        }

        if password.len() >= 12 {
            score += 1;
        }

        // Character variety checks
        if password.chars().any(|c| c.is_lowercase()) {
            score += 1;
        } else {
            feedback.push("Password should contain lowercase letters".to_string());
        }

        if password.chars().any(|c| c.is_uppercase()) {
            score += 1;
        } else {
            feedback.push("Password should contain uppercase letters".to_string());
        }

        if password.chars().any(|c| c.is_numeric()) {
            score += 1;
        } else {
            feedback.push("Password should contain numbers".to_string());
        }

        if password.chars().any(|c| !c.is_alphanumeric()) {
            score += 1;
        } else {
            feedback.push("Password should contain special characters".to_string());
        }

        // Common patterns check
        if Self::contains_common_patterns(password) {
            score = score.saturating_sub(2);
            feedback.push("Password contains common patterns".to_string());
        }

        let strength = match score {
            0..=2 => PasswordStrengthLevel::Weak,
            3..=4 => PasswordStrengthLevel::Medium,
            5..=6 => PasswordStrengthLevel::Strong,
            _ => PasswordStrengthLevel::VeryStrong,
        };

        PasswordStrength {
            level: strength,
            score,
            feedback,
        }
    }

    /// Generate a secure random password
    pub fn generate_password(length: usize) -> Result<String, PasswordError> {
        if length < 8 {
            return Err(PasswordError::InvalidLength);
        }

        if length > 128 {
            return Err(PasswordError::InvalidLength);
        }

        use uuid::Uuid;

        // Simple password generation using UUID and character sets
        let lowercase = "abcdefghijklmnopqrstuvwxyz";
        let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let numbers = "0123456789";
        let special = "!@#$%^&*()_+-=[]{}|;:,.<>?";

        let all_chars = format!("{}{}{}{}", lowercase, uppercase, numbers, special);

        // Use UUID bytes as a source of randomness
        let uuid = Uuid::new_v4();
        let uuid_bytes = uuid.as_bytes();
        let mut password = String::new();

        // Ensure at least one character from each category
        password.push(
            lowercase
                .chars()
                .nth((uuid_bytes[0] as usize) % lowercase.len())
                .unwrap(),
        );
        password.push(
            uppercase
                .chars()
                .nth((uuid_bytes[1] as usize) % uppercase.len())
                .unwrap(),
        );
        password.push(
            numbers
                .chars()
                .nth((uuid_bytes[2] as usize) % numbers.len())
                .unwrap(),
        );
        password.push(
            special
                .chars()
                .nth((uuid_bytes[3] as usize) % special.len())
                .unwrap(),
        );

        // Fill the rest with random characters
        for i in 4..length {
            let char_index = (uuid_bytes[i % 16] as usize) % all_chars.len();
            password.push(all_chars.chars().nth(char_index).unwrap());
        }

        Ok(password)
    }

    fn contains_common_patterns(password: &str) -> bool {
        let common_patterns = [
            "123456", "password", "qwerty", "abc123", "admin", "letmein", "welcome", "monkey",
            "dragon", "master", "shadow", "login",
        ];

        let lower_password = password.to_lowercase();

        for pattern in &common_patterns {
            if lower_password.contains(pattern) {
                return true;
            }
        }

        // Check for repeated characters
        if password.len() >= 3 {
            for window in password.chars().collect::<Vec<_>>().windows(3) {
                if window[0] == window[1] && window[1] == window[2] {
                    return true;
                }
            }
        }

        // Check for sequential characters
        if password.len() >= 3 {
            let chars: Vec<char> = password.chars().collect();
            for i in 0..chars.len().saturating_sub(2) {
                let c1 = chars[i] as u8;
                let c2 = chars[i + 1] as u8;
                let c3 = chars[i + 2] as u8;

                if c2 == c1 + 1 && c3 == c2 + 1 {
                    return true;
                }
            }
        }

        false
    }
}

impl PasswordStrengthLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            PasswordStrengthLevel::Weak => "Weak",
            PasswordStrengthLevel::Medium => "Medium",
            PasswordStrengthLevel::Strong => "Strong",
            PasswordStrengthLevel::VeryStrong => "Very Strong",
        }
    }
}

impl AuthService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self {
            user_repository,
            session_manager: Arc::new(SessionManager::default()),
        }
    }

    pub fn new_with_session_timeout(
        user_repository: Arc<dyn UserRepository>,
        session_timeout_hours: i64,
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

    pub fn from_global_pool_with_session_timeout(
        session_timeout_hours: i64,
    ) -> Result<Self, AuthError> {
        let user_repository = Arc::new(
            PostgresUserRepository::from_global_pool().map_err(AuthError::RepositoryError)?,
        );
        Ok(Self::new_with_session_timeout(
            user_repository,
            session_timeout_hours,
        ))
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
                    let session_info = self
                        .session_manager
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
            AuthError::SessionError(SessionError::SessionNotFound) => {
                "Session not found".to_string()
            }
            AuthError::SessionError(SessionError::SessionExpired) => "Session expired".to_string(),
            AuthError::SessionError(SessionError::SessionInactive) => {
                "Session inactive".to_string()
            }
            AuthError::SessionError(SessionError::InvalidSessionId) => {
                "Invalid session".to_string()
            }
            _ => "An internal error occurred".to_string(),
        }
    }
}
