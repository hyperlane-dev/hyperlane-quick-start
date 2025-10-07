use super::*;

#[route("/chat")]
#[utoipa::path(
    get,
    post,
    path = "/chat",
    description = "Chat frontend interface",
    responses(
        (status = 200, description = "Successfully served chat frontend interface", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(200),
    response_body(CHAT_HTML),
    response_header(CONTENT_ENCODING => GZIP)
)]
pub async fn html(ctx: Context) {}

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
pub async fn handle(ctx: Context) {
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
