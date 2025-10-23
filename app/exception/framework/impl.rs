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
        let error_message: String = error.to_string();
        log_error(&error_message).await;
        println_error!("{error_message}");
        let api_response: ApiResponse<()> =
            ApiResponse::error_with_code(ResponseCode::InternalError, error_message);
        let response_body: Vec<u8> = api_response.to_json_bytes();
        let content_type: String =
            ContentType::format_content_type_with_charset(APPLICATION_JSON, UTF8);
    }
}
