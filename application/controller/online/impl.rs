use super::*;

/// Implementation of `OnlineRoute` for `ServerHook`.
impl ServerHook for OnlineRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `OnlineRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_ws_upgrade_type, is_get_method)]
    #[instrument_trace]
    /// Handles the `OnlineRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request().get_path().clone();
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let config: WebSocketConfig<String> = WebSocketConfig::new(stream, ctx)
            .set_broadcast_type(key)
            .set_connected_hook::<OnlineConnectedHook>()
            .set_closed_hook::<OnlineClosedHook>();
        websocket.run(config).await;
        Status::Continue
    }
}
