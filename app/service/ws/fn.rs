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
    let path: String = ctx.get_request_path().await;
    let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
    let receiver_count: String = websocket
        .receiver_count_after_increment(key.clone())
        .to_string();
    let data: String = format!("{ONLINE_CONNECTIONS}{COLON_SPACE}{receiver_count}");
    let resp_data: String = WebSocketRespData::get_json_data(MessageType::OnlineCount, &ctx, data)
        .await
        .unwrap();
    spawn(async move {
        let _ = websocket.send(key, resp_data);
    });
}

pub(crate) async fn on_closed(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let path: String = ctx.get_request_path().await;
    let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
    let receiver_count: ReceiverCount = websocket.receiver_count_after_decrement(key);
    let data: String = format!("{ONLINE_CONNECTIONS}{COLON_SPACE}{receiver_count}");
    let resp_data: String = WebSocketRespData::get_json_data(MessageType::OnlineCount, &ctx, data)
        .await
        .unwrap();
    ctx.set_response_body(resp_data).await;
}

pub(crate) async fn callback(ctx: Context) {
    let req_data: WebSocketReqData = ctx
        .get_request_body_json::<WebSocketReqData>()
        .await
        .unwrap();
    if req_data.is_ping() {
        let resp_data: WebSocketRespData =
            WebSocketRespData::new(MessageType::Pang, &ctx, "").await;
        let resp_data: String = json_stringify_string(&resp_data).unwrap();
        ctx.set_response_body(resp_data)
            .await
            .send_body()
            .await
            .unwrap();
        ctx.set_response_body("").await;
        return;
    }
    let resp_data: WebSocketRespData = req_data.into_resp(&ctx).await;
    let resp_data: String = json_stringify_string(&resp_data).unwrap();
    ctx.set_response_body(resp_data).await;
}

pub(crate) async fn send_callback(_: Context) {}
