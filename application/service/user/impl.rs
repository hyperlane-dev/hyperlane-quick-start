use super::*;

impl UserService {
    #[instrument_trace]
    pub async fn get_user(user_id: i32) -> Result<Option<UserResponse>, String> {
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => Ok(Some(Self::model_to_user_response(&model)?)),
            None => Ok(None),
        }
    }

    #[instrument_trace]
    pub async fn list_users(query: UserListQueryRequest) -> Result<UserListResponse, String> {
        let limit: u64 = query.get_limit().unwrap_or(20);
        let keyword: Option<String> = query.try_get_keyword().clone();
        let last_id: Option<i32> = query
            .try_get_last_id()
            .as_ref()
            .map(|id: &String| AuthService::decode_id(id))
            .transpose()?;
        let (paged_users, total_count, has_more) =
            UserRepository::query_with_pagination(keyword, last_id, limit).await?;
        let encoded_last_id: Option<String> = paged_users
            .last()
            .map(|u: &UserModel| AuthService::encode_id(u.get_id()))
            .transpose()?;
        let user_responses: Vec<UserResponse> = paged_users
            .iter()
            .map(Self::model_to_user_response)
            .collect::<Result<Vec<UserResponse>, String>>()?;
        let mut response: UserListResponse = UserListResponse::default();
        response
            .set_users(user_responses)
            .set_has_more(has_more)
            .set_last_id(encoded_last_id)
            .set_total_count(total_count);
        Ok(response)
    }

    #[instrument_trace]
    pub async fn update_user(
        user_id: i32,
        request: UpdateUserRequest,
    ) -> Result<UserResponse, String> {
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => {
                let mut active_model: UserActiveModel = model.into();
                if let Some(email) = request.try_get_email() {
                    if !email.is_empty() && !Self::validate_email(email) {
                        return Err("Invalid email format".to_string());
                    }
                    active_model.email = ActiveValue::Set(Some(email.clone()));
                }
                if let Some(phone) = request.try_get_phone() {
                    if !phone.is_empty() && !Self::validate_phone(phone) {
                        return Err("Invalid phone format".to_string());
                    }
                    active_model.phone = ActiveValue::Set(Some(phone.clone()));
                }
                let result: UserModel = UserRepository::update(active_model).await?;
                Self::model_to_user_response(&result)
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    fn validate_email(email: &str) -> bool {
        match EMAIL_REGEX.as_ref() {
            Some(regex) => regex.is_match(email),
            None => false,
        }
    }

    #[instrument_trace]
    fn validate_phone(phone: &str) -> bool {
        match PHONE_REGEX_OPT.as_ref() {
            Some(regex) => regex.is_match(phone),
            None => false,
        }
    }

    #[instrument_trace]
    pub async fn update_user_status(user_id: i32, approved: bool) -> Result<UserResponse, String> {
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => {
                let target_role: UserRole = UserRole::try_from(model.get_role())?;
                if !approved && target_role.is_admin() {
                    return Err("Cannot reject an admin user".to_string());
                }
                let mut active_model: UserActiveModel = model.into();
                let status: i16 = if approved {
                    UserStatus::Approved.to_i16()
                } else {
                    UserStatus::Rejected.to_i16()
                };
                active_model.status = ActiveValue::Set(status);
                let result: UserModel = UserRepository::update(active_model).await?;
                Self::model_to_user_response(&result)
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
                let mut active_model: UserActiveModel = model.into();
                active_model.password_hash = ActiveValue::Set(new_password_hash);
                UserRepository::update(active_model).await?;
                Ok(())
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn delete_user(user_id: i32) -> Result<(), String> {
        match UserRepository::find_by_id(user_id).await? {
            Some(_) => {
                UserRepository::delete_by_id(user_id).await?;
                Ok(())
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    fn model_to_user_response(model: &UserModel) -> Result<UserResponse, String> {
        let mut response: UserResponse = UserResponse::default();
        let created_at: Option<i64> = model
            .try_get_created_at()
            .map(|dt| dt.and_utc().timestamp_millis());
        let role: UserRole = UserRole::try_from(model.get_role()).unwrap_or_default();
        let status: UserStatus = UserStatus::try_from(model.get_status()).unwrap_or_default();
        let encoded_id: String = AuthService::encode_id(model.get_id())?;
        response
            .set_id(encoded_id)
            .set_username(model.get_username().clone())
            .set_email(model.try_get_email().clone())
            .set_phone(model.try_get_phone().clone())
            .set_role(role.as_str().to_string())
            .set_status(status.as_str().to_string())
            .set_created_at(created_at);
        Ok(response)
    }
}
