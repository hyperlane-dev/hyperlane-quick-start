use super::*;

async fn callback(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let receiver_count: ReceiverCount = websocket
        .receiver_count(BroadcastType::PointToGroup("/"))
        .unwrap_or_default();
    let body: Vec<u8> = ctx.get_request_body().await;
    ctx.set_response_body(body).await;
    println_success!(receiver_count);
}

async fn send_callback(_: Context) {}

pub async fn handle(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    websocket
        .run(
            &ctx,
            1024,
            BroadcastType::PointToGroup("/"),
            callback,
            send_callback,
        )
        .await;
}
