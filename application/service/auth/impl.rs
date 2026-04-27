use super::*;

impl Default for AuthService {
    fn default() -> Self {
        Self {
            rsa_private_key: Arc::new(RwLock::new(None)),
            rsa_key_cache: Arc::new(RwLock::new(None)),
        }
    }
}

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
    pub fn get_auth_service() -> &'static AuthService {
        AUTH_SERVICE.get_or_init(AuthService::default)
    }

    #[instrument_trace]
    pub async fn generate_rsa_key_pair(&self) -> Result<RsaPublicKeyResponse, String> {
        if let Some(ref cache) = *self.rsa_key_cache.read().await
            && cache.created_at.elapsed().as_secs() < RSA_KEY_CACHE_TTL_SECS
        {
            let response: RsaPublicKeyResponse = serde_json::from_str(&cache.response_json)
                .map_err(|error| format!("Failed to parse cached key: {}", error))?;
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
            .map_err(|error| format!("Failed to serialize key: {}", error))?;
        let cache: RsaKeyCache = RsaKeyCache {
            response_json,
            created_at: Instant::now(),
        };
        *self.rsa_key_cache.write().await = Some(cache);
        Ok(response)
    }

    #[instrument_trace]
    async fn decrypt_rsa_field(&self, encrypted_text: &str) -> Result<String, String> {
        let key_guard: RwLockReadGuard<'_, Option<RsaPrivateKey>> =
            self.get_rsa_private_key().read().await;
        let private_key: &RsaPrivateKey = key_guard
            .as_ref()
            .ok_or_else(|| "RSA private key not initialized".to_string())?;
        let encrypted_bytes: Vec<u8> = RsaUtil::base64_decode(encrypted_text)?;
        let decrypted: String = RsaUtil::decrypt_with_private_key(private_key, &encrypted_bytes)?;
        Ok(decrypted)
    }

    #[instrument_trace]
    pub fn encode_id(id: i32) -> Result<String, String> {
        Encode::execute(CHARSETS, &id.to_string()).map_err(|_| "Failed to encode ID".to_string())
    }

    #[instrument_trace]
    pub fn decode_id(encoded_id: &str) -> Result<i32, String> {
        Decode::execute(CHARSETS, encoded_id)
            .map_err(|_| "Invalid ID format".to_string())?
            .parse::<i32>()
            .map_err(|_| "Invalid ID format".to_string())
    }

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
            return Err("Invalid email format".to_string());
        }
        if UserRepository::find_by_username(decrypted_username.clone())
            .await?
            .is_some()
        {
            return Err("Username already exists".to_string());
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
                    return Err("User is not approved".to_string());
                }
                let valid: bool =
                    PasswordUtil::verify_password(&decrypted_password, model.get_password_hash());
                if !valid {
                    return Err("Invalid password".to_string());
                }
                let user_response: UserResponse = Self::model_to_user_response(&model)?;
                let user_id: i32 = model.get_id();
                let role: i16 = model.get_role();
                Ok((user_response, user_id, role))
            }
            None => Err("User not found".to_string()),
        }
    }

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
            return Err("Invalid email format".to_string());
        }
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => {
                let mut active_model: AuthUserActiveModel = model.into();
                if decrypted_email.is_some() {
                    active_model.email = ActiveValue::Set(decrypted_email);
                }
                if decrypted_phone.is_some() {
                    active_model.phone = ActiveValue::Set(decrypted_phone);
                }
                active_model.updated_at = ActiveValue::NotSet;
                let result: AuthUserModel = UserRepository::update(active_model).await?;
                Self::model_to_user_response(&result)
            }
            None => Err("User not found".to_string()),
        }
    }

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
                    return Err("Old password is incorrect".to_string());
                }
                let new_password_hash: String =
                    PasswordUtil::hash_password(&decrypted_new_password);
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
                Self::model_to_user_response(&result)
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn list_users(query: UserListQueryRequest) -> Result<UserListResponse, String> {
        let limit: u64 = query.get_limit().unwrap_or(20);
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

    #[instrument_trace]
    pub async fn get_user(user_id: i32) -> Result<Option<UserResponse>, String> {
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => Ok(Some(Self::model_to_user_response(&model)?)),
            None => Ok(None),
        }
    }

    #[instrument_trace]
    fn model_to_user_response(model: &AuthUserModel) -> Result<UserResponse, String> {
        let mut response: UserResponse = UserResponse::default();
        let created_at: Option<i64> = model
            .try_get_created_at()
            .map(|dt| dt.and_utc().timestamp_millis());
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
