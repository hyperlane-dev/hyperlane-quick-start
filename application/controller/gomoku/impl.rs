use super::*;

impl ServerHook for GomokuRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[request_query_option("uid" => uid_opt)]
    #[prologue_macros(ws_upgrade_type, get_method)]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let websocket: &WebSocket = get_global_websocket();
        let uid: String = uid_opt.unwrap_or_default();
        let key_value: String = if uid.trim().is_empty() {
            ctx.get_request().get_path().clone()
        } else {
            uid
        };
        let key: BroadcastType<String> = BroadcastType::PointToGroup(key_value);
        let config: WebSocketConfig<String> = WebSocketConfig::new(ctx)
            .set_broadcast_type(key)
            .set_connected_hook::<GomokuConnectedHook>()
            .set_request_hook::<GomokuRequestHook>()
            .set_sended_hook::<GomokuSendedHook>()
            .set_closed_hook::<GomokuClosedHook>();
        websocket.run(config).await;
    }
}
