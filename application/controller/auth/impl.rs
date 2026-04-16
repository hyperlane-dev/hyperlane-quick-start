use super::*;

impl ServerHook for UserRegisterRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(post_method, request_body_json_result(request_opt: RegisterRequest), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let request: RegisterRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match AuthService::register_user(request).await {
            Ok(user) => {
                let response: ApiResponse<UserResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, user);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for UserLoginRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(post_method, request_body_json_result(request_opt: LoginRequest), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let request: LoginRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match AuthService::login_user(request).await {
            Ok((user_response, user_id, role)) => {
                let jwt_config: JwtConfig = JwtConfig::new(
                    JwtConfigEnum::SecretKey.to_string(),
                    JwtConfigEnum::Expiration.expiration_as_u64(),
                    JwtConfigEnum::Issuer.to_string(),
                );
                let jwt_service: JwtService = JwtService::from(jwt_config);
                let mut extra_claims: HashMap<String, serde_json::Value> = HashMap::new();
                extra_claims.insert("user_id".to_string(), json!(user_id));
                extra_claims.insert("role".to_string(), json!(role));
                let token_result: Result<JwtToken, String> = jwt_service
                    .generate_token_with_extra_claims(user_response.get_username(), extra_claims);
                match token_result {
                    Ok(jwt_token) => {
                        let token_str: String = jwt_token.get_token().to_string();
                        let cookie_value: String =
                            format!("token={token_str}; Path=/; Max-Age=86400; HttpOnly");
                        ctx.get_mut_response().set_header(SET_COOKIE, &cookie_value);
                        let mut login_response: LoginResponse = LoginResponse::default();
                        login_response.set_user(user_response).set_token(token_str);
                        let response: ApiResponse<LoginResponse> =
                            ApiResponse::new(ApiResponseStatus::Success, login_response);
                        ctx.get_mut_response().set_body(response.to_json_bytes());
                    }
                    Err(error) => {
                        let response: ApiResponse<String> =
                            ApiResponse::new(ApiResponseStatus::InternalServerError, error);
                        ctx.get_mut_response().set_body(response.to_json_bytes());
                    }
                }
            }
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
        };
    }
}

impl ServerHook for UserUpdateRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(post_method, route_param_option(ID_KEY => id_opt), request_body_json_result(request_opt: UpdateUserRequest), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let target_user_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, "Invalid user ID");
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, "User ID is required");
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user: UserResponse = match AuthService::get_user(current_user_id).await {
            Ok(Some(user_info)) => user_info,
            Ok(None) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, "User not found");
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
        if !user_role.is_admin() && current_user_id != target_user_id {
            let response: ApiResponse<&str> = ApiResponse::new(
                ApiResponseStatus::Forbidden,
                "You can only update your own data",
            );
            ctx.get_mut_response().set_body(response.to_json_bytes());
            return;
        }
        let request: UpdateUserRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match AuthService::update_user(target_user_id, request).await {
            Ok(user) => {
                let response: ApiResponse<UserResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, user);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for UserChangePasswordRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(post_method, route_param_option(ID_KEY => id_opt), request_body_json_result(request_opt: ChangePasswordRequest), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let user_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, "Invalid user ID");
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, "User ID is required");
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let request: ChangePasswordRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match AuthService::change_password(user_id, request).await {
            Ok(_) => {
                let response: ApiResponse<()> = ApiResponse::new(ApiResponseStatus::Success, ());
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for UserApproveRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(post_method, route_param_option(ID_KEY => id_opt), request_body_json_result(request_opt: ApproveUserRequest), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let user_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, "Invalid user ID");
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, "User ID is required");
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let request: ApproveUserRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match AuthService::approve_user(user_id, request.get_approved()).await {
            Ok(user) => {
                let response: ApiResponse<UserResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, user);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for UserListRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(get_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user: UserResponse = match AuthService::get_user(current_user_id).await {
            Ok(Some(user_info)) => user_info,
            Ok(None) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, "User not found");
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let mut query: UserListQueryRequest = UserListQueryRequest::default();
        let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
        if user_role.is_admin() {
            let querys: &RequestQuerys = ctx.get_request().get_querys();
            let keyword: Option<String> = querys.get("keyword").cloned();
            let last_id: Option<String> = querys.get("last_id").cloned();
            let limit: Option<u64> = querys
                .get("limit")
                .and_then(|s: &String| s.parse().ok())
                .map(|l: u64| l.min(MAX_LIMIT));
            query
                .set_keyword(keyword)
                .set_last_id(last_id)
                .set_limit(limit);
        }
        match AuthService::list_users(query).await {
            Ok(data) => {
                let response: ApiResponse<UserListResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, data);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for UserGetRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(get_method, route_param_option(ID_KEY => id_opt), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let user_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, "Invalid user ID");
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, "User ID is required");
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match AuthService::get_user(user_id).await {
            Ok(Some(user)) => {
                let response: ApiResponse<UserResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, user);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::ResourceNotFound, "User not found");
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}
