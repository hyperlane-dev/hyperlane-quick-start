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
