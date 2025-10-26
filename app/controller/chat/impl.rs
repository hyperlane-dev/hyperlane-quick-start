use super::*;

impl ServerHook for OnlineUsersRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        let user_list: UserListResponse = ChatDomain::get_online_users_list();
        let response: ApiResponse<UserListResponse> = ApiResponse::success(user_list);
        ctx.set_response_body(&response.to_json_bytes()).await;
    }
}

impl ServerHook for ChatRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(ws, get)]
    async fn handle(self, ctx: &Context) {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request_path().await;
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let cfg: WebSocketConfig<String> = WebSocketConfig::new()
            .set_context(ctx.clone())
            .set_broadcast_type(key)
            .set_buffer_size(SERVER_BUFFER)
            .set_connected_hook::<ChatConnectedHook>()
            .set_request_hook::<ChatRequestHook>()
            .set_sended_hook::<ChatSendedHook>()
            .set_closed_hook::<ChatClosedHook>();
        websocket.run(cfg).await;
    }
}

impl ServerHook for ChatHistoryRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        #[request_query("session_id" => session_id_opt)]
        async fn get_session_id(_ctx: &Context) -> Option<String> {
            session_id_opt
        }

        #[request_query("before_id" => before_id_opt)]
        async fn get_before_id(_ctx: &Context) -> Option<String> {
            before_id_opt
        }

        #[request_query("limit" => limit_opt)]
        async fn get_limit(_ctx: &Context) -> Option<String> {
            limit_opt
        }

        let session_id: String = get_session_id(ctx)
            .await
            .unwrap_or_else(|| "default".to_string());
        let before_id: Option<i64> = get_before_id(ctx).await.and_then(|s| s.parse::<i64>().ok());
        let limit: i64 = get_limit(ctx)
            .await
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(20);

        match ChatService::get_chat_history(&session_id, before_id, limit).await {
            Ok(history) => {
                let response: ApiResponse<ChatHistoryResponse> = ApiResponse::success(history);
                ctx.set_response_body(&response.to_json_bytes()).await;
            }
            Err(error) => {
                let error_response: ApiResponse<()> = ApiResponse::error(&error);
                ctx.set_response_body(&error_response.to_json_bytes()).await;
            }
        }
    }
}
