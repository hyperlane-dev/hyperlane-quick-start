use super::*;

/// Implementation of `UserListRoute` for `ServerHook`.
impl ServerHook for UserListRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_get_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let current_user: model::response::auth::UserResponse =
            match AuthService::get_user(current_user_id).await {
                Ok(Some(user_info)) => user_info,
                Ok(None) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::Unauthorized, ERROR_USER_NOT_FOUND);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
                Err(error) => {
                    let mut response: ApiResponse<String> =
                        ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                    response.set_message(&error);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
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
        match UserService::list_users(query).await {
            Ok(data) => {
                let response: ApiResponse<UserListResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, data);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `UserGetRoute` for `ServerHook`.
impl ServerHook for UserGetRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_get_method, try_get_route_param(ID_KEY => id_opt), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let user_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_INVALID_USER_ID);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_USER_ID_REQUIRED);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match UserService::get_user(user_id).await {
            Ok(Some(user)) => {
                let response: ApiResponse<UserResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, user);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::ResourceNotFound, ERROR_USER_NOT_FOUND);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `UserUpdateRoute` for `ServerHook`.
impl ServerHook for UserUpdateRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_post_method, try_get_route_param(ID_KEY => id_opt), request_body_json_result(request_opt: UpdateUserRequest), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let target_user_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_INVALID_USER_ID);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_USER_ID_REQUIRED);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let current_user: model::response::auth::UserResponse =
            match AuthService::get_user(current_user_id).await {
                Ok(Some(user_info)) => user_info,
                Ok(None) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::Unauthorized, ERROR_USER_NOT_FOUND);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
                Err(error) => {
                    let mut response: ApiResponse<String> =
                        ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                    response.set_message(&error);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            };
        let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
        if !user_role.is_admin() && current_user_id != target_user_id {
            let response: ApiResponse<&str> =
                ApiResponse::new(ApiResponseStatus::Forbidden, ERROR_UPDATE_OWN_DATA_ONLY);
            ctx.get_mut_response().set_body(response.to_json_bytes());
            return Status::Continue;
        }
        let request: UpdateUserRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match UserService::update_user(target_user_id, request).await {
            Ok(user) => {
                let response: ApiResponse<UserResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, user);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `UserChangePasswordRoute` for `ServerHook`.
impl ServerHook for UserChangePasswordRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_post_method, try_get_route_param(ID_KEY => id_opt), request_body_json_result(request_opt: ChangePasswordRequest), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let user_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_INVALID_USER_ID);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_USER_ID_REQUIRED);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let request: ChangePasswordRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match UserService::change_password(user_id, request).await {
            Ok(_) => {
                let response: ApiResponse<()> = ApiResponse::new(ApiResponseStatus::Success, ());
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `UserUpdateStatusRoute` for `ServerHook`.
impl ServerHook for UserUpdateStatusRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_post_method, try_get_route_param(ID_KEY => id_opt), request_body_json_result(request_opt: UpdateUserStatusRequest), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let user_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_INVALID_USER_ID);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_USER_ID_REQUIRED);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let request: UpdateUserStatusRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match UserService::update_user_status(user_id, request.get_approved()).await {
            Ok(user) => {
                let response: ApiResponse<UserResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, user);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `UserDeleteRoute` for `ServerHook`.
impl ServerHook for UserDeleteRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_post_method, try_get_route_param(ID_KEY => id_opt), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let current_user: model::response::auth::UserResponse =
            match AuthService::get_user(current_user_id).await {
                Ok(Some(user_info)) => user_info,
                Ok(None) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::Unauthorized, ERROR_USER_NOT_FOUND);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
                Err(error) => {
                    let mut response: ApiResponse<String> =
                        ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                    response.set_message(&error);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            };
        let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
        if !user_role.is_admin() {
            let response: ApiResponse<&str> =
                ApiResponse::new(ApiResponseStatus::Forbidden, ERROR_ONLY_ADMIN_CAN_DELETE);
            ctx.get_mut_response().set_body(response.to_json_bytes());
            return Status::Continue;
        }
        let target_user_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_INVALID_USER_ID);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_USER_ID_REQUIRED);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        if current_user_id == target_user_id {
            let response: ApiResponse<&str> =
                ApiResponse::new(ApiResponseStatus::Forbidden, ERROR_CANNOT_DELETE_YOURSELF);
            ctx.get_mut_response().set_body(response.to_json_bytes());
            return Status::Continue;
        }
        match UserService::delete_user(target_user_id).await {
            Ok(_) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Success, SUCCESS_USER_DELETED);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}
