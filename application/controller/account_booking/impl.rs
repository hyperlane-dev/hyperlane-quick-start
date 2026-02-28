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
        match AccountBookingService::register_user(request).await {
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
        match AccountBookingService::login_user(request).await {
            Ok(login_response) => {
                let response: ApiResponse<LoginResponse> =
                    ApiResponse::<LoginResponse>::success(login_response);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::Unauthorized, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
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
        match AccountBookingService::create_user(request).await {
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
        let request: UpdateUserRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match AccountBookingService::update_user(user_id, request).await {
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
        match AccountBookingService::change_password(user_id, request).await {
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
        match AccountBookingService::approve_user(user_id, request.get_approved()).await {
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
        match AccountBookingService::list_users().await {
            Ok(users) => {
                let response: ApiResponse<Vec<UserResponse>> =
                    ApiResponse::<Vec<UserResponse>>::success(users);
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
        match AccountBookingService::get_user(user_id).await {
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
        let user_id: i32 = 1;
        match AccountBookingService::create_record(user_id, request).await {
            Ok(record) => {
                let response: ApiResponse<RecordResponse> =
                    ApiResponse::<RecordResponse>::success(record);
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

impl ServerHook for RecordUpdateRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }
    #[prologue_macros(post_method, route_param_option(ID_KEY => id_opt), request_body_json_result(request_opt: UpdateRecordRequest), response_header(CONTENT_TYPE => APPLICATION_JSON))]
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
        let request: UpdateRecordRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let user_id: i32 = 1;
        match AccountBookingService::update_record(record_id, user_id, request).await {
            Ok(record) => {
                let response: ApiResponse<RecordResponse> =
                    ApiResponse::<RecordResponse>::success(record);
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
    async fn handle(self, ctx: &mut Context) {
        let mut query: RecordQueryRequest = RecordQueryRequest::default();
        if let Some(user_id_str) = user_id_opt {
            if let Ok(user_id) = user_id_str.parse::<i32>() {
                query.set_user_id(Some(user_id));
            }
        }
        if let Some(start_date_str) = start_date_opt {
            if let Ok(start_date) = NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d") {
                query.set_start_date(Some(start_date));
            }
        }
        if let Some(end_date_str) = end_date_opt {
            if let Ok(end_date) = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d") {
                query.set_end_date(Some(end_date));
            }
        }
        if let Some(category) = category_opt {
            query.set_category(Some(category));
        }
        if let Some(transaction_type) = transaction_type_opt {
            query.set_transaction_type(Some(transaction_type));
        }
        match AccountBookingService::list_records(query).await {
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
        match AccountBookingService::get_record(record_id).await {
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
