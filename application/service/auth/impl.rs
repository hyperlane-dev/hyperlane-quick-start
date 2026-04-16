use super::*;

impl PasswordUtil {
    #[instrument_trace]
    pub fn hash_password(password: &str) -> String {
        format!("{:x}", compute(password.as_bytes()))
    }

    #[instrument_trace]
    pub fn verify_password(password: &str, hash: &str) -> bool {
        Self::hash_password(password) == hash
    }
}

impl AuthService {
    #[instrument_trace]
    pub fn extract_user_from_cookie(ctx: &Context) -> Result<i32, String> {
        let token: String = match ctx.get_request().try_get_cookie(TOKEN) {
            Some(cookie) => cookie,
            None => return Err("Authentication token not found".to_string()),
        };
        let jwt_config: JwtConfig = JwtConfig::new(
            JwtConfigEnum::SecretKey.to_string(),
            JwtConfigEnum::Expiration.expiration_as_u64(),
            JwtConfigEnum::Issuer.to_string(),
        );
        let jwt_service: JwtService = JwtService::from(jwt_config);
        let user_id_value: serde_json::Value = match jwt_service.get_from_token(&token, "user_id") {
            Ok(Some(value)) => value,
            Ok(None) => return Err("user_id not found in token".to_string()),
            Err(_) => return Err("Invalid token".to_string()),
        };
        let user_id: i32 = match user_id_value.as_i64() {
            Some(id) => id as i32,
            None => return Err("Invalid user_id format in token".to_string()),
        };
        Ok(user_id)
    }

    #[instrument_trace]
    fn validate_email(email: &str) -> bool {
        let regex: Regex = match Regex::new(EMAIL_REGEX_PATTERN) {
            Ok(r) => r,
            Err(_) => return false,
        };
        regex.is_match(email)
    }

    #[instrument_trace]
    pub async fn register_user(request: RegisterRequest) -> Result<UserResponse, String> {
        if let Some(email) = request.try_get_email()
            && !email.is_empty()
            && !Self::validate_email(email)
        {
            return Err("Invalid email format".to_string());
        }
        let existing_user: Option<AuthUserModel> =
            UserRepository::find_by_username(request.get_username().clone()).await?;
        if existing_user.is_some() {
            return Err("Username already exists".to_string());
        }
        let password_hash: String = PasswordUtil::hash_password(request.get_password());
        let active_model: AuthUserActiveModel = AuthUserActiveModel {
            username: ActiveValue::Set(request.get_username().clone()),
            password_hash: ActiveValue::Set(password_hash),
            email: ActiveValue::Set(request.try_get_email().clone()),
            phone: ActiveValue::Set(request.try_get_phone().clone()),
            role: ActiveValue::Set(UserRole::User.to_i16()),
            status: ActiveValue::Set(UserStatus::Pending.to_i16()),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let result: AuthUserModel = UserRepository::insert(active_model).await?;
        Ok(Self::model_to_user_response(&result))
    }

    #[instrument_trace]
    pub async fn login_user(request: LoginRequest) -> Result<(UserResponse, i32, i16), String> {
        let user: Option<AuthUserModel> =
            UserRepository::find_by_username(request.get_username().clone()).await?;
        match user {
            Some(model) => {
                if model.get_status() != UserStatus::Approved.to_i16() {
                    return Err("User is not approved".to_string());
                }
                let valid: bool = PasswordUtil::verify_password(
                    request.get_password(),
                    model.get_password_hash(),
                );
                if !valid {
                    return Err("Invalid password".to_string());
                }
                let user_response: UserResponse = Self::model_to_user_response(&model);
                let user_id: i32 = model.get_id();
                let role: i16 = model.get_role();
                Ok((user_response, user_id, role))
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn update_user(
        user_id: i32,
        request: UpdateUserRequest,
    ) -> Result<UserResponse, String> {
        if let Some(email) = request.try_get_email()
            && !email.is_empty()
            && !Self::validate_email(email)
        {
            return Err("Invalid email format".to_string());
        }
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => {
                let mut active_model: AuthUserActiveModel = model.into();
                if let Some(email) = request.try_get_email() {
                    active_model.email = ActiveValue::Set(Some(email.clone()));
                }
                if let Some(phone) = request.try_get_phone() {
                    active_model.phone = ActiveValue::Set(Some(phone.clone()));
                }
                active_model.updated_at = ActiveValue::NotSet;
                let result: AuthUserModel = UserRepository::update(active_model).await?;
                Ok(Self::model_to_user_response(&result))
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn change_password(
        user_id: i32,
        request: ChangePasswordRequest,
    ) -> Result<(), String> {
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => {
                let valid: bool = PasswordUtil::verify_password(
                    request.get_old_password(),
                    model.get_password_hash(),
                );
                if !valid {
                    return Err("Old password is incorrect".to_string());
                }
                let new_password_hash: String =
                    PasswordUtil::hash_password(request.get_new_password());
                let mut active_model: AuthUserActiveModel = model.into();
                active_model.password_hash = ActiveValue::Set(new_password_hash);
                active_model.updated_at = ActiveValue::NotSet;
                UserRepository::update(active_model).await?;
                Ok(())
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn approve_user(user_id: i32, approved: bool) -> Result<UserResponse, String> {
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => {
                let mut active_model: AuthUserActiveModel = model.into();
                let status: i16 = if approved {
                    UserStatus::Approved.to_i16()
                } else {
                    UserStatus::Rejected.to_i16()
                };
                active_model.status = ActiveValue::Set(status);
                active_model.updated_at = ActiveValue::NotSet;
                let result: AuthUserModel = UserRepository::update(active_model).await?;
                Ok(Self::model_to_user_response(&result))
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn list_users(query: UserListQueryRequest) -> Result<UserListResponse, String> {
        let limit: u64 = query.get_limit().unwrap_or(20);
        let keyword: Option<String> = query.try_get_keyword().clone();
        let last_id: Option<i32> = query.get_last_id();
        let (paged_users, total_count, has_more) =
            UserRepository::query_with_pagination(keyword, last_id, limit).await?;
        let last_id: Option<i32> = paged_users.last().map(|u: &AuthUserModel| u.get_id());
        let user_responses: Vec<UserResponse> = paged_users
            .iter()
            .map(Self::model_to_user_response)
            .collect();
        let mut response: UserListResponse = UserListResponse::default();
        response
            .set_users(user_responses)
            .set_has_more(has_more)
            .set_last_id(last_id)
            .set_total_count(total_count);
        Ok(response)
    }

    #[instrument_trace]
    pub async fn get_user(user_id: i32) -> Result<Option<UserResponse>, String> {
        Ok(UserRepository::find_by_id(user_id)
            .await?
            .map(|model: AuthUserModel| Self::model_to_user_response(&model)))
    }

    #[instrument_trace]
    fn model_to_user_response(model: &AuthUserModel) -> UserResponse {
        let mut response: UserResponse = UserResponse::default();
        let created_at: Option<i64> = model
            .try_get_created_at()
            .map(|dt| dt.and_utc().timestamp_millis());
        let role: UserRole = UserRole::try_from(model.get_role()).unwrap_or_default();
        let status: UserStatus = UserStatus::try_from(model.get_status()).unwrap_or_default();
        response
            .set_id(model.get_id())
            .set_username(model.get_username().clone())
            .set_email(model.try_get_email().clone())
            .set_phone(model.try_get_phone().clone())
            .set_role(role.as_str().to_string())
            .set_status(status.as_str().to_string())
            .set_created_at(created_at);
        response
    }
}
