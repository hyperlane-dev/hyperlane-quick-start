use super::*;

impl ServerHook for TaskPanicHook {
    #[task_panic_data(task_panic_data)]
    #[instrument_trace]
    async fn new(ctx: &Context) -> Self {
        Self {
            content_type: ContentType::format_content_type_with_charset(APPLICATION_JSON, UTF8),
            response_body: task_panic_data.to_string(),
        }
    }

    #[prologue_macros(
        response_version(HttpVersion::Http1_1),
        response_status_code(500),
        clear_response_headers,
        response_header(SERVER => HYPERLANE),
        response_header(CONTENT_TYPE, &self.content_type),
    )]
    #[epilogue_macros(response_body(&response_body), try_send)]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        debug!("TaskPanicHook request => {}", ctx.get_request().await);
        error!("TaskPanicHook => {}", self.get_response_body());
        let api_response: ApiResponse<()> =
            ApiResponse::error_with_code(ResponseCode::InternalError, self.get_response_body());
        let response_body: Vec<u8> = api_response.to_json_bytes();
    }
}

impl ServerHook for RequestErrorHook {
    #[request_error_data(request_error_data)]
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self {
            response_status_code: request_error_data.get_http_status_code(),
            content_type: ContentType::format_content_type_with_charset(APPLICATION_JSON, UTF8),
            response_body: request_error_data.to_string(),
        }
    }

    #[prologue_macros(
        response_version(HttpVersion::Http1_1),
        response_status_code(self.get_response_status_code()),
        clear_response_headers,
        response_header(SERVER => HYPERLANE),
        response_header(CONTENT_TYPE, &self.content_type),
        response_header(TRACE => uuid::Uuid::new_v4().to_string()),
    )]
    #[epilogue_macros(response_body(&response_body), try_send)]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        if self.get_response_status_code() == HttpStatus::BadRequest.code() {
            ctx.aborted().await;
            debug!("Context aborted");
            return;
        }
        if self.get_response_status_code() != HttpStatus::RequestTimeout.code() {
            debug!("RequestErrorHook request => {}", ctx.get_request().await);
            error!("RequestErrorHook => {}", self.get_response_body());
        }
        let api_response: ApiResponse<()> =
            ApiResponse::error_with_code(ResponseCode::InternalError, self.get_response_body());
        let response_body: Vec<u8> = api_response.to_json_bytes();
    }
}
