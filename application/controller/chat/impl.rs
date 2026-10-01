use super::*;

/// Implementation of `OnlineUsersRoute` for `ServerHook`.
impl ServerHook for OnlineUsersRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `OnlineUsersRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    /// Handles the `OnlineUsersRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let user_list: UserListResponse = ChatDomain::get_online_users_list().await;
        let response: ApiResponse<UserListResponse> =
            ApiResponse::new(ApiResponseStatus::Success, user_list);
        ctx.get_mut_response().set_body(response.to_json_bytes());
        Status::Continue
    }
}

/// Implementation of `ChatRoute` for `ServerHook`.
impl ServerHook for ChatRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `ChatRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_ws_upgrade_type, is_get_method)]
    #[instrument_trace]
    /// Handles the `ChatRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request().get_path().clone();
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let config: WebSocketConfig<String> = WebSocketConfig::new(stream, ctx)
            .set_broadcast_type(key)
            .set_connected_hook::<ChatConnectedHook>()
            .set_request_hook::<ChatRequestHook>()
            .set_sended_hook::<ChatSendedHook>()
            .set_closed_hook::<ChatClosedHook>();
        websocket.run(config).await;
        Status::Continue
    }
}

/// Implementation of `ChatHistoryRoute` for `ServerHook`.
impl ServerHook for ChatHistoryRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `ChatHistoryRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        try_get_request_query(QUERY_KEY_LIMIT => limit_opt),
        try_get_request_query(QUERY_KEY_BEFORE_ID => before_id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON),
    )]
    #[instrument_trace]
    /// Handles the `ChatHistoryRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let before_id: Option<i64> = before_id_opt.and_then(|id: String| id.parse::<i64>().ok());
        let limit: u64 = limit_opt
            .and_then(|limit: String| limit.parse::<u64>().ok())
            .map(|limit: u64| limit.min(MAX_LIMIT))
            .unwrap_or(DEFAULT_CHAT_HISTORY_LIMIT);
        match ChatService::get_chat_history(before_id, limit).await {
            Ok(history) => {
                let response: ApiResponse<ChatHistoryResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, history);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
            Err(error) => {
                let error_response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InternalServerError, error);
                ctx.get_mut_response()
                    .set_body(error_response.to_json_bytes());
            }
        }
        Status::Continue
    }
}
