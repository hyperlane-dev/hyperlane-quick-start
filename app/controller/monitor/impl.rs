use super::*;

impl ServerHook for ServerStatusRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => TEXT_EVENT_STREAM)
    )]
    async fn handle(self, ctx: &Context) {
        let _ = ctx.send().await;
        loop {
            let server_status: ServerStatus = MonitorService::get_server_status().await;
            let status_json: String = serde_json::to_string(&server_status).unwrap_or_default();
            let sse_data: String = format!("data: {status_json}{DOUBLE_BR}");
            let send_result: ResponseResult =
                ctx.set_response_body(&sse_data).await.send_body().await;
            if send_result.is_err() {
                break;
            }
            sleep(Duration::from_millis(1000)).await;
        }
        let _ = ctx.closed().await;
    }
}

impl ServerHook for SystemInfoRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        let system_info: SystemInfo = MonitorService::get_system_info().await;
        let response = ApiResponse::success(system_info);
        ctx.set_response_body(&response.to_json_bytes()).await;
    }
}

impl ServerHook for NetworkCaptureRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(methods(get, post))]
    async fn handle(self, ctx: &Context) {
        MonitorService::get_network_capture_data(ctx).await;
    }
}

impl ServerHook for NetworkCaptureStreamRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => TEXT_EVENT_STREAM),
        response_header(CACHE_CONTROL => NO_CACHE),
        response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)
    )]
    async fn handle(self, ctx: &Context) {
        MonitorService::get_network_capture_stream(ctx).await;
    }
}
