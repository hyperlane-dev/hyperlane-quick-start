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
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match OrderService::register_user(request).await {
            Ok(user) => {
                let response: ApiResponse<UserResponse> =
                    ApiResponse::<UserResponse>::success(user);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
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
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match OrderService::login_user(request).await {
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
                        ctx.get_mut_response()
                            .set_header("Set-Cookie", &cookie_value);
                        let mut login_response: LoginResponse = LoginResponse::default();
                        login_response.set_user(user_response).set_token(token_str);
                        let response: ApiResponse<LoginResponse> =
                            ApiResponse::<LoginResponse>::success(login_response);
                        ctx.get_mut_response().set_body(response.to_json_bytes());
                    }
                    Err(error) => {
                        let response: ApiResponse<()> =
                            ApiResponse::<()>::error_with_code(ResponseCode::InternalError, error);
                        ctx.get_mut_response().set_body(response.to_json_bytes());
                    }
                }
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::Unauthorized, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
        };
    }
}

impl ServerHook for UserCreateRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(post_method, request_body_json_result(request_opt: CreateUserRequest), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let request: CreateUserRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match OrderService::create_user(request).await {
            Ok(user) => {
                let response: ApiResponse<UserResponse> =
                    ApiResponse::<UserResponse>::success(user);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
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
            Some(id_str) => match id_str.parse::<i32>() {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                        ResponseCode::BadRequest,
                        "Invalid user ID",
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
            },
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "User ID is required",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user_id: i32 = match OrderService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::Unauthorized, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user: UserResponse = match OrderService::get_user(current_user_id).await {
            Ok(Some(user_info)) => user_info,
            Ok(None) => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::Unauthorized,
                    "User not found",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
        if !user_role.is_admin() && current_user_id != target_user_id {
            let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                ResponseCode::Forbidden,
                "You can only update your own data",
            );
            ctx.get_mut_response().set_body(response.to_json_bytes());
            return;
        }
        let request: UpdateUserRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match OrderService::update_user(target_user_id, request).await {
            Ok(user) => {
                let response: ApiResponse<UserResponse> =
                    ApiResponse::<UserResponse>::success(user);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
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
            Some(id_str) => match id_str.parse::<i32>() {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                        ResponseCode::BadRequest,
                        "Invalid user ID",
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
            },
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "User ID is required",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let request: ChangePasswordRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match OrderService::change_password(user_id, request).await {
            Ok(_) => {
                let response: ApiResponse<()> = ApiResponse::<()>::success(());
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
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
            Some(id_str) => match id_str.parse::<i32>() {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                        ResponseCode::BadRequest,
                        "Invalid user ID",
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
            },
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "User ID is required",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let request: ApproveUserRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match OrderService::approve_user(user_id, request.get_approved()).await {
            Ok(user) => {
                let response: ApiResponse<UserResponse> =
                    ApiResponse::<UserResponse>::success(user);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
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
        let current_user_id: i32 = match OrderService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::Unauthorized, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user: UserResponse = match OrderService::get_user(current_user_id).await {
            Ok(Some(user_info)) => user_info,
            Ok(None) => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::Unauthorized,
                    "User not found",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let mut query: UserListQueryRequest = UserListQueryRequest::default();
        let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
        if user_role.is_admin() {
            let querys: &RequestQuerys = ctx.get_request().get_querys();
            let keyword: Option<String> = querys.get("keyword").cloned();
            let last_id: Option<i32> = querys.get("last_id").and_then(|s: &String| s.parse().ok());
            let limit: Option<u64> = querys
                .get("limit")
                .and_then(|s: &String| s.parse().ok())
                .map(|l: u64| l.min(MAX_LIMIT));
            query
                .set_keyword(keyword)
                .set_last_id(last_id)
                .set_limit(limit);
        }
        match OrderService::list_users(query).await {
            Ok(data) => {
                let response: ApiResponse<UserListResponse> =
                    ApiResponse::<UserListResponse>::success(data);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
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
            Some(id_str) => match id_str.parse::<i32>() {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                        ResponseCode::BadRequest,
                        "Invalid user ID",
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
            },
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "User ID is required",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match OrderService::get_user(user_id).await {
            Ok(Some(user)) => {
                let response: ApiResponse<UserResponse> =
                    ApiResponse::<UserResponse>::success(user);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::NotFound, "User not found");
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for RecordCreateRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(post_method, request_body_json_result(request_opt: CreateRecordRequest), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let request: CreateRecordRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user_id: i32 = match OrderService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::Unauthorized, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user: UserResponse = match OrderService::get_user(current_user_id).await {
            Ok(Some(user_info)) => user_info,
            Ok(None) => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::Unauthorized,
                    "User not found",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
        if !user_role.is_admin() {
            let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                ResponseCode::Forbidden,
                "Only admin can create records",
            );
            ctx.get_mut_response().set_body(response.to_json_bytes());
            return;
        }
        let target_user_id: i32 = match request.try_get_target_user_id() {
            Some(target_id) => {
                let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
                if !user_role.is_admin() {
                    let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                        ResponseCode::Forbidden,
                        "Only admin can create records for other users",
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
                target_id
            }
            None => current_user_id,
        };
        match OrderService::create_record(target_user_id, request).await {
            Ok(record) => {
                let response: ApiResponse<RecordResponse> =
                    ApiResponse::<RecordResponse>::success(record);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
        }
    }
}

impl ServerHook for RecordListRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(get_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    #[request_query_option("user_id" => user_id_opt)]
    #[request_query_option("start_date" => start_date_opt)]
    #[request_query_option("end_date" => end_date_opt)]
    #[request_query_option("category" => category_opt)]
    #[request_query_option("transaction_type" => transaction_type_opt)]
    #[request_query_option("cache_id" => cache_id_opt)]
    #[request_query_option("page" => page_opt)]
    #[request_query_option("limit" => limit_opt)]
    async fn handle(self, ctx: &mut Context) {
        let current_user_id: i32 = match OrderService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::Unauthorized, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user: UserResponse = match OrderService::get_user(current_user_id).await {
            Ok(Some(user_info)) => user_info,
            Ok(None) => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::Unauthorized,
                    "User not found",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let mut query: RecordQueryRequest = RecordQueryRequest::default();
        let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
        if user_role.is_admin() {
            if let Some(user_id_str) = user_id_opt
                && let Ok(user_id) = user_id_str.parse::<i32>()
            {
                query.set_user_id(Some(user_id));
            }
        } else {
            query.set_user_id(Some(current_user_id));
        }
        if let Some(start_date_str) = start_date_opt
            && let Ok(start_date) = NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d")
        {
            query.set_start_date(Some(start_date));
        }
        if let Some(end_date_str) = end_date_opt
            && let Ok(end_date) = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d")
        {
            query.set_end_date(Some(end_date));
        }
        if let Some(category) = category_opt {
            query.set_category(Some(category));
        }
        if let Some(transaction_type) = transaction_type_opt {
            query.set_transaction_type(Some(transaction_type));
        }
        if let Some(cache_id_str) = cache_id_opt
            && let Ok(cache_id) = cache_id_str.parse::<i32>()
        {
            query.set_cache_id(Some(cache_id));
        }
        if let Some(page_str) = page_opt
            && let Ok(page) = page_str.parse::<i32>()
        {
            query.set_page(Some(page));
        }
        if let Some(limit_str) = limit_opt
            && let Ok(limit) = limit_str.parse::<u64>()
        {
            query.set_limit(Some(limit.min(MAX_LIMIT)));
        }
        match OrderService::list_records(query).await {
            Ok(list_response) => {
                let response: ApiResponse<RecordListResponse> =
                    ApiResponse::<RecordListResponse>::success(list_response);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for RecordGetRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(get_method, route_param_option(ID_KEY => id_opt), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let record_id: i32 = match id_opt {
            Some(id_str) => match id_str.parse::<i32>() {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                        ResponseCode::BadRequest,
                        "Invalid record ID",
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
            },
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Record ID is required",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match OrderService::get_record(record_id).await {
            Ok(Some(record)) => {
                let response: ApiResponse<RecordResponse> =
                    ApiResponse::<RecordResponse>::success(record);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::NotFound, "Record not found");
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for OverviewStatisticsRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(get_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let current_user_id: i32 = match OrderService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::Unauthorized, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user: UserResponse = match OrderService::get_user(current_user_id).await {
            Ok(Some(user_info)) => user_info,
            Ok(None) => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::Unauthorized,
                    "User not found",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
        if !user_role.is_admin() {
            let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                ResponseCode::Forbidden,
                "Only admin can access overview statistics",
            );
            ctx.get_mut_response().set_body(response.to_json_bytes());
            return;
        }
        match OrderService::get_overview_statistics().await {
            Ok(statistics) => {
                let response: ApiResponse<OverviewStatisticsResponse> =
                    ApiResponse::<OverviewStatisticsResponse>::success(statistics);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for RecordCreateWithImagesRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        request_header_option("x-record-id" => record_id_opt),
        request_header_option("x-amount" => amount_opt),
        request_header_option("x-category" => category_opt),
        request_header_option("x-transaction-type" => transaction_type_opt),
        request_header_option("x-description" => description_opt),
        request_header_option("x-target-user-id" => target_user_id_opt),
        request_header_option("x-file-name" => file_name_opt),
        request_header_option("x-original-name" => original_name_opt),
        request_header_option("x-mime-type" => mime_type_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let current_user_id: i32 = match OrderService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::Unauthorized, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user: UserResponse = match OrderService::get_user(current_user_id).await {
            Ok(Some(user_info)) => user_info,
            Ok(None) => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::Unauthorized,
                    "User not found",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
        if !user_role.is_admin() {
            let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                ResponseCode::Forbidden,
                "Only admin can create records",
            );
            ctx.get_mut_response().set_body(response.to_json_bytes());
            return;
        }
        let file_name: String = match file_name_opt {
            Some(s) => s,
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Missing X-File-Name header",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let mime_type: String = match mime_type_opt {
            Some(s) => s,
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Missing X-Mime-Type header",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let file_data: Vec<u8> = ctx.get_request().get_body().clone();
        let file_size: i32 = file_data.len() as i32;
        let mut image_request: ImageUploadRequest = ImageUploadRequest::default();
        image_request
            .set_file_name(file_name)
            .set_original_name(original_name_opt)
            .set_mime_type(mime_type)
            .set_file_data(file_data)
            .set_file_size(file_size);
        let result: CreateRecordWithImagesResponse = match record_id_opt {
            Some(record_id_str) => {
                let record_id: i32 = match record_id_str.parse() {
                    Ok(id) => id,
                    Err(_) => {
                        let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                            ResponseCode::BadRequest,
                            "Invalid X-Record-Id header",
                        );
                        ctx.get_mut_response().set_body(response.to_json_bytes());
                        return;
                    }
                };
                match OrderService::add_image_to_record(record_id, current_user_id, image_request)
                    .await
                {
                    Ok(result) => result,
                    Err(error) => {
                        let response: ApiResponse<()> =
                            ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                        ctx.get_mut_response().set_body(response.to_json_bytes());
                        return;
                    }
                }
            }
            None => {
                let amount: Decimal = match amount_opt {
                    Some(s) => match s.parse() {
                        Ok(v) => v,
                        Err(_) => {
                            let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                                ResponseCode::BadRequest,
                                "Invalid amount format",
                            );
                            ctx.get_mut_response().set_body(response.to_json_bytes());
                            return;
                        }
                    },
                    None => {
                        let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                            ResponseCode::BadRequest,
                            "Missing X-Amount header",
                        );
                        ctx.get_mut_response().set_body(response.to_json_bytes());
                        return;
                    }
                };
                let category: String = match category_opt {
                    Some(s) => s,
                    None => {
                        let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                            ResponseCode::BadRequest,
                            "Missing X-Category header",
                        );
                        ctx.get_mut_response().set_body(response.to_json_bytes());
                        return;
                    }
                };
                let transaction_type: String = match transaction_type_opt {
                    Some(s) => s,
                    None => {
                        let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                            ResponseCode::BadRequest,
                            "Missing X-Transaction-Type header",
                        );
                        ctx.get_mut_response().set_body(response.to_json_bytes());
                        return;
                    }
                };
                let target_user_id: i32 = target_user_id_opt
                    .and_then(|s: String| s.parse().ok())
                    .unwrap_or(current_user_id);
                let mut record_request: CreateRecordRequest = CreateRecordRequest::default();
                record_request
                    .set_amount(amount)
                    .set_category(category)
                    .set_transaction_type(transaction_type)
                    .set_description(description_opt)
                    .set_target_user_id(Some(target_user_id));
                match OrderService::create_record_with_single_image(
                    target_user_id,
                    record_request,
                    image_request,
                )
                .await
                {
                    Ok(result) => result,
                    Err(error) => {
                        let response: ApiResponse<()> =
                            ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                        ctx.get_mut_response().set_body(response.to_json_bytes());
                        return;
                    }
                }
            }
        };
        let response: ApiResponse<CreateRecordWithImagesResponse> =
            ApiResponse::<CreateRecordWithImagesResponse>::success(result);
        ctx.get_mut_response().set_body(response.to_json_bytes());
    }
}

impl ServerHook for ImageListRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(get_method, route_param_option(RECORD_ID_KEY => record_id_opt), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let record_id: i32 = match record_id_opt {
            Some(id_str) => match id_str.parse::<i32>() {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                        ResponseCode::BadRequest,
                        "Invalid record ID",
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
            },
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Record ID is required",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match OrderService::get_record_images(record_id).await {
            Ok(images) => {
                let response: ApiResponse<RecordImageListResponse> =
                    ApiResponse::<RecordImageListResponse>::success(images);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
        };
    }
}

impl ServerHook for ImageDownloadRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(get_method, route_param_option(ID_KEY => id_opt))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let current_user_id: i32 = match OrderService::extract_user_from_cookie(ctx) {
            Ok(user_id) => user_id,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::Unauthorized, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let image_id: i32 = match id_opt {
            Some(id_str) => match id_str.parse::<i32>() {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                        ResponseCode::BadRequest,
                        "Invalid image ID",
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
            },
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Image ID is required",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match OrderService::get_image_data(image_id, current_user_id).await {
            Ok(Some(image)) => {
                let file_name: String = image
                    .try_get_original_name()
                    .clone()
                    .unwrap_or_else(|| image.get_file_name().clone());
                let content_disposition: String =
                    format!("{ATTACHMENT}; {FILENAME}=\"{}\"", file_name);
                let mime_type: String = image.get_mime_type().clone();
                let file_data: Vec<u8> = image.get_file_data().clone();
                ctx.get_mut_response()
                    .set_header(CONTENT_TYPE, &mime_type)
                    .set_header(CONTENT_DISPOSITION, &content_disposition)
                    .set_header(CONTENT_LENGTH, file_data.len().to_string())
                    .set_body(file_data);
            }
            Ok(None) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::NotFound, "Image not found");
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
        };
    }
}
