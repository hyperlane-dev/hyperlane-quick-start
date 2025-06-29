use super::*;

#[methods(get, post)]
#[utoipa::path(
    get,
    post,
    path = "/ws/index.html",   
    responses(
        (status = 200, description = "群聊前端界面", body = String)
    )
)]
#[route_param(WS_DIR_KEY => ws_path_opt)]
#[response_status_code(200)]
#[response_header(LOCATION => INDEX_HTML_URL_PATH)]
pub async fn html(ctx: Context) {
    let ws_path: String = ws_path_opt.unwrap_or_default();
    if ws_path.len() <= 3 {
        ctx.set_response_status_code(301)
            .await
            .set_response_body(vec![])
            .await;
        return;
    }
    let file_path: String = format!("./group-chat/{ws_path}");
    let extension_name: String = FileExtension::get_extension_name(&file_path);
    let content_type: &str = FileExtension::parse(&extension_name).get_content_type();
    let res: Option<Vec<u8>> = async_read_from_file(&file_path).await.ok();
    if res.is_none() {
        return;
    }
    let body: Vec<u8> = res.unwrap_or_default();
    ctx.set_response_header(CONTENT_ENCODING, GZIP)
        .await
        .set_response_header(CONTENT_TYPE, content_type)
        .await
        .set_response_body(body)
        .await;
}

#[ws]
#[get]
#[utoipa::path(
    get,
    path = "/api/ws",   
    responses(
        (status = 200, description = "群聊接口", body = WebSocketRespData)
    )
)]
pub async fn handle(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let path: String = ctx.get_request_path().await;
    let key: BroadcastType<String> = BroadcastType::PointToGroup(path);
    websocket
        .run(
            &ctx,
            SERVER_WS_BUFFER_SIZE,
            key,
            callback,
            send_callback,
            on_closed,
        )
        .await;
}
