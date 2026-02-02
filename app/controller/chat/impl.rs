use super::*;

impl ServerHook for OnlineUsersRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let user_list: UserListResponse = ChatDomain::get_online_users_list().await;
        let response: ApiResponse<UserListResponse> = ApiResponse::success(user_list);
        ctx.set_response_body(&response.to_json_bytes()).await;
    }
}

impl ServerHook for ChatRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(ws_upgrade_type, get_method)]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request_path().await;
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let cfg: WebSocketConfig<String> = WebSocketConfig::new()
            .set_context(ctx.clone())
            .set_broadcast_type(key)
            .set_request_config_data(RequestConfigData::default())
            .set_connected_hook::<ChatConnectedHook>()
            .set_request_hook::<ChatRequestHook>()
            .set_sended_hook::<ChatSendedHook>()
            .set_closed_hook::<ChatClosedHook>();
        websocket.run(cfg).await;
    }
}

impl ServerHook for ChatHistoryRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        #[request_query_option("before_id" => before_id_opt)]
        async fn get_before_id(ctx: &Context) -> Option<String> {
            before_id_opt
        }
        #[request_query_option("limit" => limit_opt)]
        async fn get_limit(_tx: &Context) -> Option<String> {
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
                ctx.set_response_body(&response.to_json_bytes()).await;
            }
            Err(error) => {
                let error_response: ApiResponse<()> = ApiResponse::error(&error);
                ctx.set_response_body(&error_response.to_json_bytes()).await;
            }
        }
    }
}
