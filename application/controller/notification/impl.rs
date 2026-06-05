use super::*;

impl ServerHook for NotificationCreateRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        request_body_json_result(request_opt: CreateNotificationRequest),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let request: CreateNotificationRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
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
        match NotificationService::create_notification(current_user_id, request).await {
            Ok(notification) => {
                let response: ApiResponse<NotificationResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, notification);
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

impl ServerHook for NotificationListRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_get_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    #[try_get_request_query("notification_type" => notification_type_opt)]
    #[try_get_request_query("is_read" => is_read_opt)]
    #[try_get_request_query("page" => page_opt)]
    #[try_get_request_query("limit" => limit_opt)]
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
        let mut query: NotificationListQueryRequest = NotificationListQueryRequest::default();
        if let Some(notification_type) = notification_type_opt {
            query.set_notification_type(Some(notification_type));
        }
        if let Some(is_read_str) = is_read_opt
            && let Ok(is_read) = is_read_str.parse::<bool>()
        {
            query.set_is_read(Some(is_read));
        }
        if let Some(page_str) = page_opt
            && let Ok(page) = page_str.parse::<i32>()
        {
            query.set_page(Some(page));
        }
        if let Some(limit_str) = limit_opt
            && let Ok(limit) = limit_str.parse::<u64>()
        {
            query.set_limit(Some(limit.min(MAX_LIST_LIMIT)));
        }
        match NotificationService::list_notifications(current_user_id, query).await {
            Ok(list_response) => {
                let response: ApiResponse<NotificationListResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, list_response);
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

impl ServerHook for NotificationGetRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        try_get_route_param(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let notification_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> = ApiResponse::new(
                        ApiResponseStatus::InvalidRequest,
                        ERROR_INVALID_NOTIFICATION_ID,
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    ERROR_NOTIFICATION_ID_REQUIRED,
                );
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
        match NotificationService::get_notification(notification_id, current_user_id).await {
            Ok(Some(notification)) => {
                let response: ApiResponse<NotificationResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, notification);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::ResourceNotFound,
                    ERROR_NOTIFICATION_NOT_FOUND,
                );
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

impl ServerHook for NotificationReadRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        try_get_route_param(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let notification_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> = ApiResponse::new(
                        ApiResponseStatus::InvalidRequest,
                        ERROR_INVALID_NOTIFICATION_ID,
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    ERROR_NOTIFICATION_ID_REQUIRED,
                );
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
        match NotificationService::mark_as_read(notification_id, current_user_id).await {
            Ok(()) => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::Success,
                    SUCCESS_NOTIFICATION_MARKED_AS_READ,
                );
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

impl ServerHook for NotificationReadAllRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_post_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
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
        match NotificationService::mark_all_as_read(current_user_id).await {
            Ok(()) => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::Success,
                    SUCCESS_ALL_NOTIFICATIONS_MARKED_AS_READ,
                );
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

impl ServerHook for NotificationDeleteRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        try_get_route_param(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let notification_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> = ApiResponse::new(
                        ApiResponseStatus::InvalidRequest,
                        ERROR_INVALID_NOTIFICATION_ID,
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    ERROR_NOTIFICATION_ID_REQUIRED,
                );
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
        match NotificationService::delete_notification(notification_id, current_user_id).await {
            Ok(()) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Success, SUCCESS_NOTIFICATION_DELETED);
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

impl ServerHook for NotificationUnreadCountRoute {
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
        match NotificationService::get_unread_count(current_user_id).await {
            Ok(count) => {
                let mut unread_response: UnreadCountResponse = UnreadCountResponse::default();
                unread_response.set_count(count);
                let response: ApiResponse<UnreadCountResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, unread_response);
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
