use super::*;

#[panic_hook]
#[epilogue_hooks(
    response_status_code(500),
    response_body(&response_body),
    response_header(SERVER => HYPERLANE),
    response_version(HttpVersion::HTTP1_1),
    response_header(CONTENT_TYPE, &content_type),
    send
)]
pub async fn panic_hook(ctx: Context) {
    let error: Panic = ctx.try_get_panic().await.unwrap_or_default();
    let response_body: String = error.to_string();
    log_error(&response_body).await;
    println_error!(response_body);
    let content_type: String = ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8);
    let _ = ctx.clear_response_headers().await;
}
