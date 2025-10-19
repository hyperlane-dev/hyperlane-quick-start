use super::*;

impl ServerHook for PanicHook {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        clear_response_headers,
        response_status_code(500),
        response_body(&response_body),
        response_header(SERVER => HYPERLANE),
        response_version(HttpVersion::HTTP1_1),
        response_header(CONTENT_TYPE, &content_type),
        send
    )]
    async fn handle(self, ctx: &Context) {
        let error: Panic = ctx.try_get_panic().await.unwrap_or_default();
        let response_body: String = error.to_string();
        log_error(&response_body).await;
        println_error!("{response_body}");
        let content_type: String = ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8);
    }
}
