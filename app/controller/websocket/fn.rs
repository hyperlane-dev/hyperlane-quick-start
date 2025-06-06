use super::*;

async fn callback(ctx: Context) {
    let body: Vec<u8> = ctx.get_request_body().await;
    ctx.set_response_body(body).await;
}

async fn send_callback(_: Context) {}

async fn client_closed_callback(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let receiver_count: ReceiverCount = websocket
        .pre_decrement_receiver_count(BroadcastType::PointToGroup("/"))
        .unwrap_or_default();
    ctx.set_response_body(format!("Current online client count: {}", receiver_count))
        .await;
}

pub async fn handle(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    websocket
        .run(
            &ctx,
            1_024_000,
            BroadcastType::PointToGroup("/"),
            callback,
            send_callback,
            client_closed_callback,
        )
        .await;
}
