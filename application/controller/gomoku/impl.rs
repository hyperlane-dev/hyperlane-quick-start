use super::*;

impl ServerHook for GomokuRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(ws_upgrade_type, get_method)]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let websocket: &WebSocket = get_global_websocket();
        let user_id: String = GomokuWebSocketService::get_user_id(ctx).await;
        let key_value: String = if user_id.trim().is_empty() {
            ctx.get_request_path().await
        } else {
            user_id
        };
        let key: BroadcastType<String> = BroadcastType::PointToGroup(key_value);
        let config: WebSocketConfig<String> = WebSocketConfig::new()
            .set_context(ctx.clone())
            .set_broadcast_type(key)
            .set_request_config_data(RequestConfigData::default())
            .set_connected_hook::<GomokuConnectedHook>()
            .set_request_hook::<GomokuRequestHook>()
            .set_sended_hook::<GomokuSendedHook>()
            .set_closed_hook::<GomokuClosedHook>();
        websocket.run(config).await;
    }
}
