use super::*;

impl ServerHook for NotificationCreateRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        request_body_json_result(request_opt: CreateNotificationRequest),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let request: CreateNotificationRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
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
    }
}

impl ServerHook for NotificationListRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(get_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    #[request_query_option("notification_type" => notification_type_opt)]
    #[request_query_option("is_read" => is_read_opt)]
    #[request_query_option("page" => page_opt)]
    #[request_query_option("limit" => limit_opt)]
    async fn handle(self, ctx: &mut Context) {
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
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
            query.set_limit(Some(limit.min(100)));
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
    }
}

impl ServerHook for NotificationGetRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        route_param_option(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let notification_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> = ApiResponse::new(
                        ApiResponseStatus::InvalidRequest,
                        "Invalid notification ID",
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
            },
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    "Notification ID is required",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
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
                    "Notification not found",
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
    }
}

impl ServerHook for NotificationReadRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        route_param_option(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let notification_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> = ApiResponse::new(
                        ApiResponseStatus::InvalidRequest,
                        "Invalid notification ID",
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
            },
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    "Notification ID is required",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match NotificationService::mark_as_read(notification_id, current_user_id).await {
            Ok(()) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Success, "Notification marked as read");
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for NotificationReadAllRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(post_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match NotificationService::mark_all_as_read(current_user_id).await {
            Ok(()) => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::Success,
                    "All notifications marked as read",
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
    }
}

impl ServerHook for NotificationDeleteRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        route_param_option(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let notification_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> = ApiResponse::new(
                        ApiResponseStatus::InvalidRequest,
                        "Invalid notification ID",
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return;
                }
            },
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    "Notification ID is required",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match NotificationService::delete_notification(notification_id, current_user_id).await {
            Ok(()) => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::Success,
                    "Notification deleted successfully",
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
    }
}

impl ServerHook for NotificationUnreadCountRoute {
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
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
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
    }
}
