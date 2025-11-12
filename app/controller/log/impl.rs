use super::*;

impl ServerHook for InfoLogRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
        response_header(CONTENT_ENCODING => GZIP)
    )]
    async fn handle(self, ctx: &Context) {
        let log_content: String = LogService::read_log_file(SERVER_LOG_LEVEL[0]).await;
        ctx.set_response_body(&log_content).await;
    }
}

impl ServerHook for WarnLogRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
        response_header(CONTENT_ENCODING => GZIP)
    )]
    async fn handle(self, ctx: &Context) {
        let log_content: String = LogService::read_log_file(SERVER_LOG_LEVEL[1]).await;
        ctx.set_response_body(&log_content).await;
    }
}

impl ServerHook for ErrorLogRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
        response_header(CONTENT_ENCODING => GZIP)
    )]
    async fn handle(self, ctx: &Context) {
        let log_content: String = LogService::read_log_file(SERVER_LOG_LEVEL[2]).await;
        ctx.set_response_body(log_content).await;
    }
}
