use super::*;

pub async fn handle(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let key: BroadcastType<'_> = BroadcastType::PointToGroup("/");
    websocket
        .run(&ctx, 1_024_000, key, callback, send_callback, on_closed)
        .await;
}
