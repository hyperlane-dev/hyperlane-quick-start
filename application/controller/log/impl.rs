use super::*;

impl ServerHook for TraceLogRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
        response_header(CONTENT_ENCODING => GZIP)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let log_content: String = LogService::read_log_file(Level::Trace).await;
        ctx.get_mut_response().set_body(&log_content);
    }
}

impl ServerHook for DebugLogRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
        response_header(CONTENT_ENCODING => GZIP)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let log_content: String = LogService::read_log_file(Level::Debug).await;
        ctx.get_mut_response().set_body(&log_content);
    }
}

impl ServerHook for InfoLogRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
        response_header(CONTENT_ENCODING => GZIP)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let log_content: String = LogService::read_log_file(Level::Info).await;
        ctx.get_mut_response().set_body(&log_content);
    }
}

impl ServerHook for WarnLogRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
        response_header(CONTENT_ENCODING => GZIP)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let log_content: String = LogService::read_log_file(Level::Warn).await;
        ctx.get_mut_response().set_body(&log_content);
    }
}

impl ServerHook for ErrorLogRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
        response_header(CONTENT_ENCODING => GZIP)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let log_content: String = LogService::read_log_file(Level::Error).await;
        ctx.get_mut_response().set_body(log_content);
    }
}
