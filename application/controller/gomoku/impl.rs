use super::*;

/// Implementation of `GomokuRoute` for `ServerHook`.
impl ServerHook for GomokuRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `GomokuRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[try_get_request_query("uid" => uid_opt)]
    #[prologue_macros(is_ws_upgrade_type, is_get_method)]
    #[instrument_trace]
    /// Handles the `GomokuRoute` route request and writes the JSON
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
        let uid: String = uid_opt.unwrap_or_default();
        let key_value: String = if uid.trim().is_empty() {
            ctx.get_request().get_path().clone()
        } else {
            uid
        };
        let key: BroadcastType<String> = BroadcastType::PointToGroup(key_value);
        let config: WebSocketConfig<String> = WebSocketConfig::new(stream, ctx)
            .set_broadcast_type(key)
            .set_connected_hook::<GomokuConnectedHook>()
            .set_request_hook::<GomokuRequestHook>()
            .set_sended_hook::<GomokuSendedHook>()
            .set_closed_hook::<GomokuClosedHook>();
        websocket.run(config).await;
        Status::Continue
    }
}
