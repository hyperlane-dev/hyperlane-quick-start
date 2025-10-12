use super::*;

#[route("/chat/users/online")]
#[utoipa::path(
    get,
    path = "/chat/users/online",
    responses(
        (status = 200, description = "Get online users list", body = UserListResponse)
    )
)]
#[prologue_macros(
    get,
    response_status_code(200),
    response_header(CONTENT_TYPE => APPLICATION_JSON)
)]
pub async fn online_users(ctx: Context) {
    let user_list: UserListResponse = get_online_users_list();
    let response = ApiResponse::success(user_list);
    ctx.set_response_body(&response.to_json_bytes()).await;
}

#[route("/api/chat")]
#[utoipa::path(
    get,
    path = "/api/chat",
    description = "WebSocket API for chat functionality",
    responses(
        (status = 200, description = "Successfully established chat WebSocket connection", body = WebSocketRespData)
    )
)]
#[prologue_macros(ws, get)]
pub async fn chat(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let path: String = ctx.get_request_path().await;
    let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
    let cfg: WebSocketConfig<String> = WebSocketConfig::new()
        .set_context(ctx.clone())
        .set_broadcast_type(key)
        .set_buffer_size(SERVER_BUFFER)
        .set_request_hook(callback)
        .set_sended_hook(send_callback)
        .set_closed_hook(on_closed);
    websocket.run(cfg).await;
}
