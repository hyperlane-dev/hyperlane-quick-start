use super::*;

impl ServerHook for OnlineRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(ws, get)]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request_path().await;
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let cfg: WebSocketConfig<String> = WebSocketConfig::new()
            .set_context(ctx.clone())
            .set_broadcast_type(key)
            .set_request_config_data(RequestConfigData::default())
            .set_connected_hook::<OnlineConnectedHook>()
            .set_closed_hook::<OnlineClosedHook>();
        websocket.run(cfg).await;
    }
}
