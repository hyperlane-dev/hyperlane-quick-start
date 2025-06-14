use super::*;

#[methods(get, post)]
pub async fn html(ctx: Context) {
    let ws_path: String = ctx.get_route_param(WS_DIR_KEY).await.unwrap_or_default();
    if ws_path.len() <= 3 {
        ctx.set_response_status_code(301)
            .await
            .set_response_header(LOCATION, INDEX_HTML_URL_PATH)
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
    ctx.set_response_status_code(200)
        .await
        .set_response_header(CONTENT_TYPE, content_type)
        .await
        .set_response_body(body)
        .await;
}

#[get]
pub async fn handle(ctx: Context) {
    let websocket: &WebSocket = get_global_websocket();
    let key: BroadcastType<'_> = BroadcastType::PointToGroup("/");
    websocket
        .run(&ctx, 1_024_000, key, callback, send_callback, on_closed)
        .await;
}
