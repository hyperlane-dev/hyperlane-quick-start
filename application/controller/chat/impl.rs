use super::*;

impl ServerHook for OnlineUsersRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let user_list: UserListResponse = ChatDomain::get_online_users_list().await;
        let response: ApiResponse<UserListResponse> = ApiResponse::success(user_list);
        ctx.get_mut_response().set_body(response.to_json_bytes());
    }
}

impl ServerHook for ChatRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(ws_upgrade_type, get_method)]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request().get_path().clone();
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let config: WebSocketConfig<String> = WebSocketConfig::new(ctx)
            .set_broadcast_type(key)
            .set_connected_hook::<ChatConnectedHook>()
            .set_request_hook::<ChatRequestHook>()
            .set_sended_hook::<ChatSendedHook>()
            .set_closed_hook::<ChatClosedHook>();
        websocket.run(config).await;
    }
}

impl ServerHook for ChatHistoryRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        #[request_query_option("before_id" => before_id_opt)]
        async fn get_before_id(ctx: &mut Context) -> Option<String> {
            before_id_opt
        }
        #[request_query_option("limit" => limit_opt)]
        async fn get_limit(_tx: &mut Context) -> Option<String> {
            limit_opt
        }
        let before_id: Option<i64> = get_before_id(ctx)
            .await
            .and_then(|id| id.parse::<i64>().ok());
        let limit: i64 = get_limit(ctx)
            .await
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(20);
        match ChatService::get_chat_history(before_id, limit).await {
            Ok(history) => {
                let response: ApiResponse<ChatHistoryResponse> = ApiResponse::success(history);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
            Err(error) => {
                let error_response: ApiResponse<()> = ApiResponse::error(&error);
                ctx.get_mut_response()
                    .set_body(error_response.to_json_bytes());
            }
        }
    }
}
