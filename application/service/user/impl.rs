use super::*;

/// Implementation of methods for `UserService`.
impl UserService {
    /// Retrieves a user by ID and converts the model to a response.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user ID.
    ///
    /// # Returns
    ///
    /// - `Result<Option<UserResponse>, String>`: The user response if found, or `None`.
    #[instrument_trace]
    pub async fn get_user(user_id: i32) -> Result<Option<UserResponse>, String> {
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => Ok(Some(Self::model_to_user_response(&model)?)),
            None => Ok(None),
        }
    }

    /// Lists users with cursor-based pagination and optional keyword filtering.
    ///
    /// # Arguments
    ///
    /// - `UserListQueryRequest`: The query parameters including keyword, last ID, and limit.
    ///
    /// # Returns
    ///
    /// - `Result<UserListResponse, String>`: The paginated user list response with encoded IDs.
    #[instrument_trace]
    pub async fn list_users(query: UserListQueryRequest) -> Result<UserListResponse, String> {
        let limit: u64 = query.get_limit().unwrap_or(DEFAULT_PAGE_LIMIT);
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

    /// Updates a user's email and phone after validating their formats.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user ID.
    /// - `UpdateUserRequest`: The update request containing optional email and phone.
    ///
    /// # Returns
    ///
    /// - `Result<UserResponse, String>`: The updated user response, or an error if not found or validation fails.
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
                        return Err(ERROR_INVALID_EMAIL_FORMAT.to_string());
                    }
                    active_model.email = ActiveValue::Set(Some(email.clone()));
                }
                if let Some(phone) = request.try_get_phone() {
                    if !phone.is_empty() && !Self::validate_phone(phone) {
                        return Err(ERROR_INVALID_PHONE_FORMAT.to_string());
                    }
                    active_model.phone = ActiveValue::Set(Some(phone.clone()));
                }
                let result: UserModel = UserRepository::update(active_model).await?;
                Self::model_to_user_response(&result)
            }
            None => Err(ERROR_USER_NOT_FOUND.to_string()),
        }
    }

    /// Validates the format of an email address using a compiled regex.
    ///
    /// # Arguments
    ///
    /// - `&str`: The email address to validate.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if valid, `false` otherwise.
    #[instrument_trace]
    fn validate_email(email: &str) -> bool {
        match EMAIL_REGEX.as_ref() {
            Some(regex) => regex.is_match(email),
            None => false,
        }
    }

    /// Validates the format of a phone number using a compiled regex.
    ///
    /// # Arguments
    ///
    /// - `&str`: The phone number to validate.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if valid, `false` otherwise.
    #[instrument_trace]
    fn validate_phone(phone: &str) -> bool {
        match PHONE_REGEX_OPT.as_ref() {
            Some(regex) => regex.is_match(phone),
            None => false,
        }
    }

    /// Updates a user's status to approved or rejected, preventing rejection of admin users.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user ID.
    /// - `bool`: `true` to approve, `false` to reject.
    ///
    /// # Returns
    ///
    /// - `Result<UserResponse, String>`: The updated user response, or an error if the user is not found or is an admin being rejected.
    #[instrument_trace]
    pub async fn update_user_status(user_id: i32, approved: bool) -> Result<UserResponse, String> {
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => {
                let target_role: UserRole = UserRole::try_from(model.get_role())?;
                if !approved && target_role.is_admin() {
                    return Err(ERROR_CANNOT_REJECT_ADMIN.to_string());
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
            None => Err(ERROR_USER_NOT_FOUND.to_string()),
        }
    }

    /// Changes a user's password after verifying the old password.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user ID.
    /// - `ChangePasswordRequest`: The request containing old and new passwords.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the old password is incorrect or user is not found.
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
                    return Err(ERROR_OLD_PASSWORD_INCORRECT.to_string());
                }
                let new_password_hash: String =
                    PasswordUtil::hash_password(request.get_new_password());
                let mut active_model: UserActiveModel = model.into();
                active_model.password_hash = ActiveValue::Set(new_password_hash);
                UserRepository::update(active_model).await?;
                Ok(())
            }
            None => Err(ERROR_USER_NOT_FOUND.to_string()),
        }
    }

    /// Deletes a user by ID after verifying the user exists.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user ID.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the user is not found.
    #[instrument_trace]
    pub async fn delete_user(user_id: i32) -> Result<(), String> {
        match UserRepository::find_by_id(user_id).await? {
            Some(_) => {
                UserRepository::delete_by_id(user_id).await?;
                Ok(())
            }
            None => Err(ERROR_USER_NOT_FOUND.to_string()),
        }
    }

    /// Converts a `UserModel` to a `UserResponse` with encoded ID and formatted fields.
    ///
    /// # Arguments
    ///
    /// - `&UserModel`: The database model to convert.
    ///
    /// # Returns
    ///
    /// - `Result<UserResponse, String>`: The converted user response, or an error if ID encoding fails.
    #[instrument_trace]
    fn model_to_user_response(model: &UserModel) -> Result<UserResponse, String> {
        let mut response: UserResponse = UserResponse::default();
        let created_at: Option<i64> = model
            .try_get_created_at()
            .map(|dt: NaiveDateTime| dt.and_utc().timestamp_millis());
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
