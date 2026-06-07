use super::*;

/// Implementation of `AuthService` for `Default`.
impl Default for AuthService {
    fn default() -> Self {
        Self {
            rsa_private_key: Arc::new(RwLock::new(None)),
            rsa_key_cache: Arc::new(RwLock::new(None)),
        }
    }
}

/// Implementation of methods for `PasswordUtil`.
impl PasswordUtil {
    /// Hashes a password using the compute digest algorithm and returns a hexadecimal string.
    ///
    /// # Arguments
    ///
    /// - `&str`: The plaintext password to hash.
    ///
    /// # Returns
    ///
    /// - `String`: The hexadecimal hash string of the password.
    #[instrument_trace]
    pub fn hash_password(password: &str) -> String {
        format!("{:x}", compute(password.as_bytes()))
    }

    /// Verifies a plaintext password against a stored hash by comparing their hash values.
    ///
    /// # Arguments
    ///
    /// - `&str`: The plaintext password to verify.
    /// - `&str`: The stored hash to compare against.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the password matches the hash, `false` otherwise.
    #[instrument_trace]
    pub fn verify_password(password: &str, hash: &str) -> bool {
        Self::hash_password(password) == hash
    }
}

/// Implementation of methods for `AuthService`.
impl AuthService {
    /// Returns the singleton `AuthService` instance, initializing it on first access.
    ///
    /// # Returns
    ///
    /// - `&'static AuthService`: The static reference to the global auth service instance.
    #[instrument_trace]
    pub fn get_auth_service() -> &'static AuthService {
        AUTH_SERVICE.get_or_init(AuthService::default)
    }

    /// Generates an RSA key pair and returns the public key in JWK format, caching the result.
    ///
    /// If a cached key pair exists and has not expired, returns the cached response.
    /// Otherwise, generates a new key pair, stores the private key in memory, and caches the response.
    ///
    /// # Returns
    ///
    /// - `Result<RsaPublicKeyResponse, String>`: The public key response with modulus and exponent, or an error message.
    #[instrument_trace]
    pub async fn generate_rsa_key_pair(&self) -> Result<RsaPublicKeyResponse, String> {
        if let Some(ref cache) = *self.rsa_key_cache.read().await
            && cache.created_at.elapsed().as_secs() < RSA_KEY_CACHE_TTL_SECS
        {
            let response: RsaPublicKeyResponse = serde_json::from_str(&cache.response_json)
                .map_err(|error: serde_json::Error| {
                    format!("Failed to parse cached key: {}", error)
                })?;
            return Ok(response);
        }
        let (private_key, public_key): (RsaPrivateKey, rsa::RsaPublicKey) =
            RsaUtil::generate_key_pair()?;
        let (n_b64, e_b64): (String, String) = RsaUtil::public_key_to_jwk(&public_key)?;
        {
            let mut key_guard = self.rsa_private_key.write().await;
            *key_guard = Some(private_key);
        }
        let mut response: RsaPublicKeyResponse = RsaPublicKeyResponse::default();
        response.set_modulus(n_b64).set_exponent(e_b64);
        let response_json: String = serde_json::to_string(&response)
            .map_err(|error: serde_json::Error| format!("Failed to serialize key: {}", error))?;
        let cache: RsaKeyCache = RsaKeyCache {
            response_json,
            created_at: Instant::now(),
        };
        *self.rsa_key_cache.write().await = Some(cache);
        Ok(response)
    }

    /// Decrypts an RSA-encrypted field using the stored private key.
    ///
    /// # Arguments
    ///
    /// - `&str`: The base64-encoded encrypted text to decrypt.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>`: The decrypted plaintext, or an error if the private key is not initialized.
    #[instrument_trace]
    async fn decrypt_rsa_field(&self, encrypted_text: &str) -> Result<String, String> {
        let key_guard: RwLockReadGuard<'_, Option<RsaPrivateKey>> =
            self.get_rsa_private_key().read().await;
        let private_key: &RsaPrivateKey = key_guard
            .as_ref()
            .ok_or_else(|| ERROR_RSA_PRIVATE_KEY_NOT_INITIALIZED.to_string())?;
        let encrypted_bytes: Vec<u8> = RsaUtil::base64_decode(encrypted_text)?;
        let decrypted: String = RsaUtil::decrypt_with_private_key(private_key, &encrypted_bytes)?;
        Ok(decrypted)
    }

    /// Encodes a numeric ID into an obfuscated string using the configured character set.
    ///
    /// # Arguments
    ///
    /// - `i32`: The numeric ID to encode.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>`: The encoded string, or an error if encoding fails.
    #[instrument_trace]
    pub fn encode_id(id: i32) -> Result<String, String> {
        Encode::execute(CHARSETS, &id.to_string())
            .map_err(|_: EncodeError| ERROR_FAILED_TO_ENCODE_ID.to_string())
    }

    /// Decodes an obfuscated string back to the original numeric ID.
    ///
    /// # Arguments
    ///
    /// - `&str`: The encoded ID string.
    ///
    /// # Returns
    ///
    /// - `Result<i32, String>`: The decoded numeric ID, or an error if the format is invalid.
    #[instrument_trace]
    pub fn decode_id(encoded_id: &str) -> Result<i32, String> {
        Decode::execute(CHARSETS, encoded_id)
            .map_err(|_: DecodeError| ERROR_INVALID_ID_FORMAT.to_string())?
            .parse::<i32>()
            .map_err(|_: std::num::ParseIntError| ERROR_INVALID_ID_FORMAT.to_string())
    }

    /// Extracts the user ID from the authentication token stored in the request cookie.
    ///
    /// # Arguments
    ///
    /// - `&Context`: The request context containing the cookie.
    ///
    /// # Returns
    ///
    /// - `Result<i32, String>`: The extracted user ID, or an error if the token is missing or invalid.
    #[instrument_trace]
    pub fn extract_user_from_cookie(ctx: &Context) -> Result<i32, String> {
        let token: String = match ctx.get_request().try_get_cookie(TOKEN) {
            Some(cookie) => cookie,
            None => return Err(ERROR_AUTHENTICATION_TOKEN_NOT_FOUND.to_string()),
        };
        let jwt_config: JwtConfig = JwtConfig::new(
            JwtConfigEnum::SecretKey.to_string(),
            JwtConfigEnum::Expiration.expiration_as_u64(),
            JwtConfigEnum::Issuer.to_string(),
        );
        let jwt_service: JwtService = JwtService::from(jwt_config);
        let user_id_value: serde_json::Value =
            match jwt_service.get_from_token(&token, JWT_CLAIM_USER_ID) {
                Ok(Some(value)) => value,
                Ok(None) => return Err(ERROR_USER_ID_NOT_FOUND_IN_TOKEN.to_string()),
                Err(_) => return Err(ERROR_INVALID_TOKEN.to_string()),
            };
        let user_id: i32 = match user_id_value.as_i64() {
            Some(id) => id as i32,
            None => return Err(ERROR_INVALID_USER_ID_FORMAT_IN_TOKEN.to_string()),
        };
        Ok(user_id)
    }

    /// Validates the format of an email address using a compiled regex.
    ///
    /// # Arguments
    ///
    /// - `&str`: The email address to validate.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the email matches the pattern, `false` otherwise or if the regex is not available.
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
    /// - `bool`: `true` if the phone matches the pattern, `false` otherwise or if the regex is not available.
    #[instrument_trace]
    fn validate_phone(phone: &str) -> bool {
        match PHONE_REGEX_OPT.as_ref() {
            Some(regex) => regex.is_match(phone),
            None => false,
        }
    }

    /// Registers a new user by decrypting RSA-encrypted credentials, validating them, and persisting to the database.
    ///
    /// # Arguments
    ///
    /// - `RegisterRequest`: The registration request containing encrypted username, password, email, and phone.
    ///
    /// # Returns
    ///
    /// - `Result<UserResponse, String>`: The created user response, or an error if validation or persistence fails.
    #[instrument_trace]
    pub async fn register_user(&self, request: RegisterRequest) -> Result<UserResponse, String> {
        let decrypted_password: String = self.decrypt_rsa_field(request.get_password()).await?;
        let decrypted_username: String = self.decrypt_rsa_field(request.get_username()).await?;
        let decrypted_email: Option<String> = match request.try_get_email() {
            Some(email) if !email.is_empty() => Some(self.decrypt_rsa_field(email).await?),
            _ => None,
        };
        let decrypted_phone: Option<String> = match request.try_get_phone() {
            Some(phone) if !phone.is_empty() => Some(self.decrypt_rsa_field(phone).await?),
            _ => None,
        };
        if let Some(ref email) = decrypted_email
            && !email.is_empty()
            && !Self::validate_email(email)
        {
            return Err(ERROR_INVALID_EMAIL_FORMAT.to_string());
        }
        if let Some(ref phone) = decrypted_phone
            && !phone.is_empty()
            && !Self::validate_phone(phone)
        {
            return Err(ERROR_INVALID_PHONE_FORMAT.to_string());
        }
        if UserRepository::find_by_username(decrypted_username.clone())
            .await?
            .is_some()
        {
            return Err(ERROR_USERNAME_ALREADY_EXISTS.to_string());
        }
        let password_hash: String = PasswordUtil::hash_password(&decrypted_password);
        let active_model: AuthUserActiveModel = AuthUserActiveModel {
            username: ActiveValue::Set(decrypted_username),
            password_hash: ActiveValue::Set(password_hash),
            email: ActiveValue::Set(decrypted_email),
            phone: ActiveValue::Set(decrypted_phone),
            role: ActiveValue::Set(UserRole::User.to_i16()),
            status: ActiveValue::Set(UserStatus::Pending.to_i16()),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let result: AuthUserModel = UserRepository::insert(active_model).await?;
        Self::model_to_user_response(&result)
    }

    /// Authenticates a user by decrypting RSA-encrypted credentials and verifying against the database.
    ///
    /// # Arguments
    ///
    /// - `LoginRequest`: The login request containing encrypted username and password.
    ///
    /// # Returns
    ///
    /// - `Result<(UserResponse, i32, i16), String>`: A tuple of (user response, user ID, role) on success, or an error.
    #[instrument_trace]
    pub async fn login_user(
        &self,
        request: LoginRequest,
    ) -> Result<(UserResponse, i32, i16), String> {
        let decrypted_password: String = self.decrypt_rsa_field(request.get_password()).await?;
        let decrypted_username: String = self.decrypt_rsa_field(request.get_username()).await?;
        let user: Option<AuthUserModel> =
            UserRepository::find_by_username(decrypted_username.clone()).await?;
        match user {
            Some(model) => {
                if model.get_status() != UserStatus::Approved.to_i16() {
                    return Err(ERROR_USER_IS_NOT_APPROVED.to_string());
                }
                let valid: bool =
                    PasswordUtil::verify_password(&decrypted_password, model.get_password_hash());
                if !valid {
                    return Err(ERROR_INVALID_PASSWORD.to_string());
                }
                let user_response: UserResponse = Self::model_to_user_response(&model)?;
                let user_id: i32 = model.get_id();
                let role: i16 = model.get_role();
                Ok((user_response, user_id, role))
            }
            None => Err(ERROR_USER_NOT_FOUND.to_string()),
        }
    }

    /// Updates a user's email and phone after decrypting RSA-encrypted fields and validating formats.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user ID.
    /// - `UpdateUserRequest`: The update request containing encrypted email and phone.
    ///
    /// # Returns
    ///
    /// - `Result<UserResponse, String>`: The updated user response, or an error if the user is not found or validation fails.
    #[instrument_trace]
    pub async fn update_user(
        &self,
        user_id: i32,
        request: UpdateUserRequest,
    ) -> Result<UserResponse, String> {
        let decrypted_email: Option<String> = match request.try_get_email() {
            Some(email) if !email.is_empty() => Some(self.decrypt_rsa_field(email).await?),
            _ => None,
        };
        let decrypted_phone: Option<String> = match request.try_get_phone() {
            Some(phone) if !phone.is_empty() => Some(self.decrypt_rsa_field(phone).await?),
            _ => None,
        };
        if let Some(ref email) = decrypted_email
            && !email.is_empty()
            && !Self::validate_email(email)
        {
            return Err(ERROR_INVALID_EMAIL_FORMAT.to_string());
        }
        if let Some(ref phone) = decrypted_phone
            && !phone.is_empty()
            && !Self::validate_phone(phone)
        {
            return Err(ERROR_INVALID_PHONE_FORMAT.to_string());
        }
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => {
                let mut active_model: AuthUserActiveModel = model.into();
                if let Some(email) = decrypted_email {
                    active_model.email = ActiveValue::Set(Some(email));
                }
                if let Some(phone) = decrypted_phone {
                    active_model.phone = ActiveValue::Set(Some(phone));
                }
                let result: AuthUserModel = UserRepository::update(active_model).await?;
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
    /// - `ChangePasswordRequest`: The request containing encrypted old and new passwords.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error if the old password is incorrect or user is not found.
    #[instrument_trace]
    pub async fn change_password(
        &self,
        user_id: i32,
        request: ChangePasswordRequest,
    ) -> Result<(), String> {
        let decrypted_old_password: String =
            self.decrypt_rsa_field(request.get_old_password()).await?;
        let decrypted_new_password: String =
            self.decrypt_rsa_field(request.get_new_password()).await?;
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => {
                let valid: bool = PasswordUtil::verify_password(
                    &decrypted_old_password,
                    model.get_password_hash(),
                );
                if !valid {
                    return Err(ERROR_OLD_PASSWORD_IS_INCORRECT.to_string());
                }
                let new_password_hash: String =
                    PasswordUtil::hash_password(&decrypted_new_password);
                let mut active_model: AuthUserActiveModel = model.into();
                active_model.password_hash = ActiveValue::Set(new_password_hash);
                UserRepository::update(active_model).await?;
                Ok(())
            }
            None => Err(ERROR_USER_NOT_FOUND.to_string()),
        }
    }

    /// Updates a user's status to approved or rejected based on the `approved` flag.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user ID.
    /// - `bool`: `true` to approve, `false` to reject.
    ///
    /// # Returns
    ///
    /// - `Result<UserResponse, String>`: The updated user response, or an error if the user is not found.
    #[instrument_trace]
    pub async fn update_user_status(user_id: i32, approved: bool) -> Result<UserResponse, String> {
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => {
                let mut active_model: AuthUserActiveModel = model.into();
                let status: i16 = if approved {
                    UserStatus::Approved.to_i16()
                } else {
                    UserStatus::Rejected.to_i16()
                };
                active_model.status = ActiveValue::Set(status);
                let result: AuthUserModel = UserRepository::update(active_model).await?;
                Self::model_to_user_response(&result)
            }
            None => Err(ERROR_USER_NOT_FOUND.to_string()),
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
            .map(|id: &String| Self::decode_id(id))
            .transpose()?;
        let (paged_users, total_count, has_more) =
            UserRepository::query_with_pagination(keyword, last_id, limit).await?;
        let encoded_last_id: Option<String> = paged_users
            .last()
            .map(|u: &AuthUserModel| Self::encode_id(u.get_id()))
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

    /// Retrieves a single user by ID and returns the user response.
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

    /// Converts an `AuthUserModel` to a `UserResponse` with encoded ID and formatted fields.
    ///
    /// # Arguments
    ///
    /// - `&AuthUserModel`: The database model to convert.
    ///
    /// # Returns
    ///
    /// - `Result<UserResponse, String>`: The converted user response, or an error if ID encoding fails.
    #[instrument_trace]
    fn model_to_user_response(model: &AuthUserModel) -> Result<UserResponse, String> {
        let mut response: UserResponse = UserResponse::default();
        let created_at: Option<i64> = model
            .try_get_created_at()
            .map(|dt: NaiveDateTime| dt.and_utc().timestamp_millis());
        let role: UserRole = UserRole::try_from(model.get_role()).unwrap_or_default();
        let status: UserStatus = UserStatus::try_from(model.get_status()).unwrap_or_default();
        let encoded_id: String = Self::encode_id(model.get_id())?;
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
