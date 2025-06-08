use super::*;

pub(crate) fn get_global_websocket() -> &'static WebSocket {
    GLOBAL_WEBSOCKET.get_or_init(|| WebSocket::new())
}

pub async fn on_connected(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let key: BroadcastType<'_> = BroadcastType::PointToGroup("/");
    let receiver_count: String = websocket.receiver_count_after_increment(key).to_string();
    let resp_data: String =
        WebSocketRespData::get_json_data(MessageType::OnlineCount, &ctx, receiver_count)
            .await
            .unwrap();
    spawn(async move {
        let _ = websocket.send(key, json_stringify_string(&resp_data).unwrap());
    });
}

pub(crate) async fn on_closed(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let key: BroadcastType<'_> = BroadcastType::PointToGroup("/");
    let receiver_count: ReceiverCount = websocket.receiver_count_after_decrement(key);
    let resp_data: String =
        WebSocketRespData::get_json_data(MessageType::OnlineCount, &ctx, receiver_count)
            .await
            .unwrap();
    ctx.set_response_body(resp_data).await;
}

pub(crate) async fn callback(ctx: Context) {
    let data: WebSocketRespData = ctx
        .get_request_body_json::<WebSocketReqData>()
        .await
        .unwrap()
        .into_resp(&ctx)
        .await;
    let resp_data: String = json_stringify_string(&data).unwrap();
    ctx.set_response_body(resp_data).await;
}

pub(crate) async fn send_callback(_: Context) {}
