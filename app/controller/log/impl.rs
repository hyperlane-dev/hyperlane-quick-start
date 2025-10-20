use super::*;

#[utoipa::path(
    get,
    path = "/api/log/info",
    responses(
        (status = 200, description = "Get info logs")
    )
)]
pub async fn info() {}

#[utoipa::path(
    get,
    path = "/api/log/warn",
    responses(
        (status = 200, description = "Get warn logs")
    )
)]
pub async fn warn() {}

#[utoipa::path(
    get,
    path = "/api/log/error",
    responses(
        (status = 200, description = "Get error logs")
    )
)]
pub async fn error() {}

impl ServerHook for InfoLogRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
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
        response_status_code(200),
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
        response_status_code(200),
        response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
        response_header(CONTENT_ENCODING => GZIP)
    )]
    async fn handle(self, ctx: &Context) {
        let log_content: String = LogService::read_log_file(SERVER_LOG_LEVEL[2]).await;
        ctx.set_response_body(log_content).await;
    }
}
