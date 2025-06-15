use super::*;

pub(crate) fn get_global_websocket() -> &'static WebSocket {
    GLOBAL_WEBSOCKET.get_or_init(|| WebSocket::new())
}

pub async fn before_ws_upgrade(ctx: Context) {
    let addr: String = ctx.get_socket_addr_or_default_string().await;
    let encode_addr: String = Encode::execute(CHARSETS, &addr).unwrap_or_default();
    ctx.set_response_header("addr", encode_addr).await;
}

pub async fn on_connected(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    spawn(async move {
        let path: String = ctx.get_request_path().await;
        let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
        let receiver_count: String = websocket
            .receiver_count_after_increment(key.clone())
            .to_string();
        let resp_data: String =
            WebSocketRespData::get_json_data(MessageType::OnlineCount, &ctx, receiver_count)
                .await
                .unwrap();
        let _ = websocket.send(key, resp_data);
    });
}

pub(crate) async fn on_closed(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let path: String = ctx.get_request_path().await;
    let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
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
