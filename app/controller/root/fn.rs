use super::*;

pub async fn handle(ctx: Context) {
    let dir_name: String = ctx.get_route_param(DIR_KEY).await.unwrap_or_default();
    let file_name: String = ctx.get_route_param(FILE_KEY).await.unwrap_or_default();
    if dir_name == INDEX_HTML_FILE_NAME {
        ctx.set_response_status_code(301)
            .await
            .set_response_header(LOCATION, INDEX_HTML_URL_PATH)
            .await
            .set_response_body(vec![])
            .await;
        return;
    }
    if dir_name.is_empty() {
        ctx.set_response_status_code(200)
            .await
            .set_response_body(INDEX_HTML)
            .await
            .set_response_header(CONTENT_TYPE, TEXT_HTML)
            .await;
        return;
    }
    let file_path: String = if file_name.is_empty() {
        format!("./group-chat/dist/{dir_name}")
    } else {
        format!("./group-chat/dist/{dir_name}/{file_name}")
    };
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
