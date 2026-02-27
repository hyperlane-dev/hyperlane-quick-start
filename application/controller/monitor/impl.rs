use super::*;

impl ServerHook for ServerStatusRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_header(CONTENT_TYPE => TEXT_EVENT_STREAM),
        response_body(vec![]),
        send
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        loop {
            let server_status: ServerStatus = MonitorService::get_server_status().await;
            let status_json: String = serde_json::to_string(&server_status).unwrap_or_default();
            let sse_data: String = format!("data: {status_json}{BR}{BR}");
            ctx.get_mut_response().set_body(&sse_data);
            if ctx.try_send_body().await.is_err() {
                break;
            }
            sleep(Duration::from_millis(1000)).await;
        }
        ctx.set_closed(true);
    }
}

impl ServerHook for SystemInfoRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let system_info: SystemInfo = MonitorService::get_system_info().await;
        let response: ApiResponse<SystemInfo> = ApiResponse::success(system_info);
        ctx.get_mut_response().set_body(response.to_json_bytes());
    }
}

impl ServerHook for NetworkCaptureRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(methods(get, post))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        MonitorService::get_network_capture_data(ctx).await;
    }
}

impl ServerHook for NetworkCaptureStreamRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => TEXT_EVENT_STREAM),
        response_header(CACHE_CONTROL => NO_CACHE),
        response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        MonitorService::get_network_capture_stream(ctx).await;
    }
}

impl ServerHook for PerformanceHistoryRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let history_response: PerformanceHistoryResponse =
            MonitorService::get_performance_history_response().await;
        let response: ApiResponse<PerformanceHistoryResponse> =
            ApiResponse::success(history_response);
        ctx.get_mut_response().set_body(response.to_json_bytes());
    }
}
