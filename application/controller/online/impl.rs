use super::*;

impl ServerHook for OnlineRoute {
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
            .set_connected_hook::<OnlineConnectedHook>()
            .set_closed_hook::<OnlineClosedHook>();
        websocket.run(config).await;
    }
}
