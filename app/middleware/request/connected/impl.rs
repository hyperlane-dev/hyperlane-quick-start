use super::*;

impl ServerHook for ConnectedMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(ws)]
    async fn handle(self, ctx: &Context) {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request_path().await;
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let receiver_count: String = websocket
            .receiver_count_after_increment(key.clone())
            .to_string();
        let username: String = ChatService::get_name(ctx).await;
        ChatDomain::add_online_user(&username);
        let resp_data: ResponseBody =
            ChatService::create_online_count_message(&ctx, receiver_count).await;
        ctx.set_response_body(&resp_data).await;
        ChatService::broadcast_online_count(key, resp_data);
    }
}
