use super::*;

impl ServerHook for TaskPanicHook {
    #[prologue_macros(task_panic_data(task_panic_data))]
    async fn new(ctx: &Context) -> Self {
        let content_type: String =
            ContentType::format_content_type_with_charset(APPLICATION_JSON, UTF8);
        Self {
            content_type,
            response_body: task_panic_data.to_string(),
        }
    }

    #[epilogue_macros(
        response_version(HttpVersion::Http1_1),
        response_status_code(500),
        clear_response_headers,
        response_body(&response_body),
        response_header(SERVER => HYPERLANE),
        response_version(HttpVersion::Http1_1),
        response_header(CONTENT_TYPE, &self.content_type),
        send
    )]
    async fn handle(self, ctx: &Context) {
        log_error(&self.response_body).await;
        let api_response: ApiResponse<()> =
            ApiResponse::error_with_code(ResponseCode::InternalError, self.response_body);
        let response_body: Vec<u8> = api_response.to_json_bytes();
    }
}

impl ServerHook for RequestErrorHook {
    #[prologue_macros(request_error_data(request_error_data))]
    async fn new(_ctx: &Context) -> Self {
        let content_type: String =
            ContentType::format_content_type_with_charset(APPLICATION_JSON, UTF8);
        Self {
            response_status_code: request_error_data.get_http_status_code(),
            content_type,
            response_body: request_error_data.to_string(),
        }
    }

    #[epilogue_macros(
        response_version(HttpVersion::Http1_1),
        response_status_code(self.response_status_code),
        clear_response_headers,
        response_body(&response_body),
        response_header(SERVER => HYPERLANE),
        response_version(HttpVersion::Http1_1),
        response_header(CONTENT_TYPE, &self.content_type),
        send
    )]
    async fn handle(self, ctx: &Context) {
        if self.response_status_code == HttpStatus::BadRequest.code() {
            ctx.aborted().await;
            return;
        }
        if self.response_status_code != HttpStatus::RequestTimeout.code() {
            log_error(&self.response_body).await;
        }
        let api_response: ApiResponse<()> =
            ApiResponse::error_with_code(ResponseCode::InternalError, self.response_body);
        let response_body: Vec<u8> = api_response.to_json_bytes();
    }
}
