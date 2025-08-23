use super::*;

#[utoipa::path(
    get,
    post,
    path = "/chat/index.html",   
    responses(
        (status = 200, description = "Chat frontend interface", body = String)
    )
)]
#[methods(get, post)]
#[route("/{ws_dir:^chat.*}")]
#[route_param(WS_DIR_KEY => request_path_opt)]
#[response_header(LOCATION => INDEX_HTML_URL_PATH)]
pub async fn html(ctx: Context) {
    let request_path: String = request_path_opt.unwrap_or_default();
    if request_path.len() <= 5 {
        ctx.set_response_status_code(301)
            .await
            .set_response_header(LOCATION, INDEX_HTML_URL_PATH)
            .await
            .set_response_body(vec![])
            .await;
        return;
    }
    let file_path: String = format!("./chat/{request_path}");
    let extension_name: String = FileExtension::get_extension_name(&file_path);
    let content_type: &str = FileExtension::parse(&extension_name).get_content_type();
    let res: Option<Vec<u8>> = async_read_from_file(&file_path).await.ok();
    if res.is_none() {
        return;
    }
    let body: Vec<u8> = res.unwrap_or_default();
    ctx.set_response_status_code(200)
        .await
        .set_response_header(CONTENT_ENCODING, GZIP)
        .await
        .set_response_header(CONTENT_TYPE, content_type)
        .await
        .set_response_body(body)
        .await;
}

#[ws]
#[get]
#[route("/api/chat")]
#[disable_ws_hook("/api/chat")]
#[utoipa::path(
    get,
    path = "/api/chat",   
    responses(
        (status = 200, description = "Chat API", body = WebSocketRespData)
    )
)]
pub async fn handle(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let path: String = ctx.get_request_path().await;
    let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
    let cfg: WebSocketConfig<String> = WebSocketConfig::new()
        .set_context(ctx.clone())
        .set_broadcast_type(key)
        .set_buffer_size(SERVER_WS_BUFFER)
        .set_request_hook(callback)
        .set_sended_hook(send_callback)
        .set_closed_hook(on_closed);
    websocket.run(cfg).await;
}
