use super::*;

impl ServerHook for OnlineUsersRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let user_list: UserListResponse = ChatDomain::get_online_users_list().await;
        let response: ApiResponse<UserListResponse> =
            ApiResponse::new(ApiResponseStatus::Success, user_list);
        ctx.get_mut_response().set_body(response.to_json_bytes());
        Status::Continue
    }
}

impl ServerHook for ChatRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_ws_upgrade_type, is_get_method)]
    #[instrument_trace]
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

impl ServerHook for ChatHistoryRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        try_get_request_query("limit" => limit_opt),
        try_get_request_query("before_id" => before_id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON),
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let before_id: Option<i64> = before_id_opt.and_then(|id| id.parse::<i64>().ok());
        let limit: u64 = limit_opt
            .and_then(|s| s.parse::<u64>().ok())
            .map(|l: u64| l.min(MAX_LIMIT))
            .unwrap_or(20);
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
