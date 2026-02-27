use super::*;

impl ServerHook for OnlineConnectedHook {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request().get_path().clone();
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let receiver_count: ReceiverCount = websocket.receiver_count(key.clone());
        OnlineService::broadcast_online_count(key, receiver_count).await;
    }
}

impl ServerHook for OnlineClosedHook {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let websocket: &WebSocket = get_global_websocket();
        let path: String = ctx.get_request().get_path().clone();
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let receiver_count: ReceiverCount = websocket.receiver_count_after_closed(key.clone());
        OnlineService::broadcast_online_count(key, receiver_count).await;
    }
}

impl OnlineService {
    #[instrument_trace]
    async fn broadcast_online_count(key: BroadcastType<String>, count: ReceiverCount) {
        let websocket: &WebSocket = get_global_websocket();
        let message: String = format!(r#"{{"type":"online_count","count":{count}}}"#);
        let message_bytes: Vec<u8> = message.into_bytes();
        let _: Result<Option<ReceiverCount>, SendError<Vec<u8>>> =
            websocket.try_send(key, message_bytes);
    }
}
