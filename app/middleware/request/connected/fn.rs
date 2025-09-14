use super::*;

#[ws]
#[request_middleware(7)]
pub async fn connected_hook(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let path: String = ctx.get_request_path().await;
    let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
    let receiver_count: String = websocket
        .receiver_count_after_increment(key.clone())
        .to_string();
    let username: String = get_name(&ctx).await;
    add_online_user(&username);
    let resp_data: ResponseBody = create_online_count_message(&ctx, receiver_count).await;
    let _ = ctx.set_response_body(&resp_data).await.send_body().await;
    broadcast_online_count(key, resp_data);
}
