use super::*;

pub async fn on_connected(_: Context) {
    tokio::spawn(async move {
        let websocket: &WebSocket = get_global_websocket();
        let key: BroadcastType<'_> = BroadcastType::PointToGroup("/");
        let receiver_count: ReceiverCount = websocket.receiver_count(key).unwrap_or_default();
        let data: String = format!("Current online client count: {}", receiver_count);
        websocket.send(key, data).unwrap();
    });
}

async fn on_closed(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let key: BroadcastType<'_> = BroadcastType::PointToGroup("/");
    let receiver_count: ReceiverCount = websocket
        .pre_decrement_receiver_count(key)
        .unwrap_or_default();
    let data: String = format!("Current online client count: {}", receiver_count);
    ctx.set_response_body(data).await;
}

async fn callback(ctx: Context) {
    let body: Vec<u8> = ctx.get_request_body().await;
    ctx.set_response_body(body).await;
}

async fn send_callback(_: Context) {}

pub async fn handle(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let key: BroadcastType<'_> = BroadcastType::PointToGroup("/");
    websocket
        .run(&ctx, 1_024_000, key, callback, send_callback, on_closed)
        .await;
}
