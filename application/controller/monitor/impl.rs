use super::*;

impl ServerHook for ServerStatusRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_header(CONTENT_TYPE => TEXT_EVENT_STREAM),
        send
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        loop {
            let server_status: ServerStatus = MonitorService::get_server_status().await;
            let status_json: String = serde_json::to_string(&server_status).unwrap_or_default();
            let sse_data: String = format!("data {status_json}{DOUBLE_BR}");
            let send_result: Result<(), ResponseError> =
                ctx.set_response_body(&sse_data).await.try_send_body().await;
            if send_result.is_err() {
                break;
            }
            sleep(Duration::from_millis(1000)).await;
        }
        ctx.closed().await;
    }
}

impl ServerHook for SystemInfoRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let system_info: SystemInfo = MonitorService::get_system_info().await;
        let response: ApiResponse<SystemInfo> = ApiResponse::success(system_info);
        ctx.set_response_body(&response.to_json_bytes()).await;
    }
}

impl ServerHook for NetworkCaptureRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(methods(get, post))]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        MonitorService::get_network_capture_data(ctx).await;
    }
}

impl ServerHook for NetworkCaptureStreamRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => TEXT_EVENT_STREAM),
        response_header(CACHE_CONTROL => NO_CACHE),
        response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        MonitorService::get_network_capture_stream(ctx).await;
    }
}
