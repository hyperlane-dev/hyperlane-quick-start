use super::*;

pub async fn panic_hook(ctx: Context) {
    let error: Panic = ctx.get_panic().await.unwrap_or_default();
    let response_body: String = error.to_string();
    println_error!(response_body);
    log_error(response_body.clone()).await;
    let content_type: String = ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8);
    let _ = ctx
        .set_response_version(HttpVersion::HTTP1_1)
        .await
        .set_response_status_code(500)
        .await
        .clear_response_headers()
        .await
        .replace_response_header(SERVER, HYPERLANE)
        .await
        .replace_response_header(CONTENT_TYPE, content_type)
        .await
        .set_response_body(response_body)
        .await
        .send()
        .await;
}
