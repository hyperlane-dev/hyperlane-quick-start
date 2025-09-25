use super::*;
use chrono::{DateTime, Utc};

impl User {
    pub fn new(
        id: i64,
        username: String,
        password_hash: String,
        email: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        is_active: bool,
    ) -> Self {
        Self {
            id,
            username,
            password_hash,
            email,
            created_at,
            updated_at,
            is_active,
        }
    }

    pub fn is_valid_username(username: &str) -> bool {
        !username.is_empty()
            && username.len() >= 3
            && username.len() <= 255
            && username
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    }

    pub fn is_valid_email(email: &str) -> bool {
        email.contains('@') && email.len() <= 255
    }

    pub fn sanitize_for_response(&self) -> UserResponse {
        UserResponse {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            is_active: self.is_active,
        }
    }
}

impl CreateUserRequest {
    pub fn validate(&self) -> Result<(), UserValidationError> {
        if !User::is_valid_username(&self.username) {
            return Err(UserValidationError::InvalidUsername);
        }

        if self.password.len() < 6 {
            return Err(UserValidationError::PasswordTooShort);
        }

        if self.password.len() > 128 {
            return Err(UserValidationError::PasswordTooLong);
        }

        if let Some(ref email) = self.email {
            if !User::is_valid_email(email) {
                return Err(UserValidationError::InvalidEmail);
            }
        }

        Ok(())
    }
}

impl UpdateUserRequest {
    pub fn validate(&self) -> Result<(), UserValidationError> {
        if let Some(ref email) = self.email {
            if !User::is_valid_email(email) {
                return Err(UserValidationError::InvalidEmail);
            }
        }
        Ok(())
    }
}

impl LoginRequest {
    pub fn validate(&self) -> Result<(), UserValidationError> {
        // 验证用户名
        if self.username.trim().is_empty() {
            return Err(UserValidationError::EmptyUsername);
        }

        if self.username.len() < 3 || self.username.len() > 255 {
            return Err(UserValidationError::InvalidUsername);
        }

        // 验证用户名格式（只允许字母、数字、下划线和连字符）
        if !self
            .username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(UserValidationError::InvalidUsername);
        }

        // 验证密码
        if self.password.is_empty() {
            return Err(UserValidationError::EmptyPassword);
        }

        if self.password.len() < 6 {
            return Err(UserValidationError::PasswordTooShort);
        }

        if self.password.len() > 128 {
            return Err(UserValidationError::PasswordTooLong);
        }

        Ok(())
    }

    /// 创建新的登录请求，自动去除用户名的首尾空白字符
    pub fn new(username: String, password: String) -> Self {
        Self {
            username: username.trim().to_string(),
            password,
        }
    }
}

impl LoginResponse {
    pub fn success(user_id: i64, username: String) -> Self {
        Self {
            success: true,
            message: "Login successful".to_string(),
            user_id: Some(user_id),
            username: Some(username),
            session_id: None,
            expires_at: None,
        }
    }

    pub fn failure(message: String) -> Self {
        Self {
            success: false,
            message,
            user_id: None,
            username: None,
            session_id: None,
            expires_at: None,
        }
    }

    /// 设置会话信息
    pub fn with_session(
        mut self,
        session_id: String,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.session_id = Some(session_id);
        self.expires_at = Some(expires_at);
        self
    }
}

impl ChangePasswordRequest {
    pub fn validate(&self) -> Result<(), UserValidationError> {
        if self.current_password.is_empty() {
            return Err(UserValidationError::EmptyPassword);
        }

        if self.new_password.len() < 6 {
            return Err(UserValidationError::PasswordTooShort);
        }

        if self.new_password.len() > 128 {
            return Err(UserValidationError::PasswordTooLong);
        }

        if self.current_password == self.new_password {
            return Err(UserValidationError::SamePassword);
        }

        Ok(())
    }
}
