use super::*;

impl ServerHook for OnlineRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("OnlineRoute new");
        Self
    }

    #[prologue_macros(ws, get)]
    async fn handle(self, ctx: &Context) {
        trace!("OnlineRoute handle");
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request_path().await;
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let cfg: WebSocketConfig<String> = WebSocketConfig::new()
            .set_context(ctx.clone())
            .set_broadcast_type(key)
            .set_request_config(RequestConfig::default())
            .set_connected_hook::<OnlineConnectedHook>()
            .set_closed_hook::<OnlineClosedHook>();
        websocket.run(cfg).await;
    }
}
