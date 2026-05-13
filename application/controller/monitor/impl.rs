use super::*;

impl ServerHook for ServerStatusRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        response_header(CONTENT_TYPE => TEXT_EVENT_STREAM),
        response_body(vec![]),
        try_send
    )]
    #[instrument_trace]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        loop {
            let server_status: ServerStatus = MonitorService::get_server_status().await;
            let status_json: String = serde_json::to_string(&server_status).unwrap_or_default();
            let sse_data: String = format!("data: {status_json}{BR}{BR}");
            ctx.get_mut_response().set_body(&sse_data);
            if stream
                .try_send(ctx.get_mut_response().build())
                .await
                .is_err()
            {
                break;
            }
            sleep(Duration::from_secs(MONITOR_INTERVAL_SECONDS)).await;
        }
        stream.set_closed(true);
        Status::Reject
    }
}

impl ServerHook for SystemInfoRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let system_info: SystemInfo = MonitorService::get_system_info().await;
        let response: ApiResponse<SystemInfo> =
            ApiResponse::new(ApiResponseStatus::Success, system_info);
        ctx.get_mut_response().set_body(response.to_json_bytes());
        Status::Continue
    }
}

impl ServerHook for NetworkCaptureRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(methods(get, post))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        MonitorService::get_network_capture_data(ctx).await;
        Status::Continue
    }
}

impl ServerHook for NetworkCaptureStreamRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => TEXT_EVENT_STREAM),
        response_header(CACHE_CONTROL => NO_CACHE),
        response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        MonitorService::get_network_capture_stream(ctx).await;
        Status::Continue
    }
}

impl ServerHook for PerformanceHistoryRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let history_response: PerformanceHistoryResponse =
            MonitorService::get_performance_history_response().await;
        let response: ApiResponse<PerformanceHistoryResponse> =
            ApiResponse::new(ApiResponseStatus::Success, history_response);
        ctx.get_mut_response().set_body(response.to_json_bytes());
        Status::Continue
    }
}
