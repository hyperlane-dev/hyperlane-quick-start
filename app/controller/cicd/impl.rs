use super::*;

impl ServerHook for CreatePipelineRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        request_body_json_result(param: CreatePipelineParam),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let param: CreatePipelineParam = match param {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match CicdService::create_pipeline(param).await {
            Ok(id) => {
                let response: ApiResponse<i32> = ApiResponse::success(id);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
        };
    }
}

impl ServerHook for ListPipelinesRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        match CicdService::get_all_pipelines().await {
            Ok(pipelines) => {
                let response: ApiResponse<Vec<PipelineDto>> = ApiResponse::success(pipelines);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
        };
    }
}

impl ServerHook for GetPipelineRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let querys: RequestQuerys = ctx.get_request_querys().await;
        let id: i32 = match querys.get("id").and_then(|s: &String| s.parse().ok()) {
            Some(id) => id,
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Missing or invalid id parameter",
                );
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match CicdService::get_pipeline_by_id(id).await {
            Ok(Some(pipeline)) => {
                let response: ApiResponse<PipelineDto> = ApiResponse::success(pipeline);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Ok(None) => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::NotFound,
                    "Pipeline not found",
                );
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
        };
    }
}

impl ServerHook for TriggerRunRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        request_body_json_result(param: TriggerRunParam),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let param: TriggerRunParam = match param {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match CicdService::trigger_run(param).await {
            Ok(id) => {
                let response: ApiResponse<i32> = ApiResponse::success(id);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
        };
    }
}

impl ServerHook for ListRunsRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let querys: RequestQuerys = ctx.get_request_querys().await;
        let page_size: Option<i32> = querys
            .get("page_size")
            .and_then(|s: &String| s.parse().ok());
        let last_id: Option<i32> = querys.get("last_id").and_then(|s: &String| s.parse().ok());
        let pipeline_id: Option<i32> = querys
            .get("pipeline_id")
            .and_then(|s: &String| s.parse().ok());
        let param: QueryRunsParam = QueryRunsParam {
            pipeline_id,
            status: None,
            page_size,
            last_id,
        };
        match CicdService::query_runs(param).await {
            Ok(result) => {
                let response: ApiResponse<PaginatedRunsDto> = ApiResponse::success(result);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
        };
    }
}

impl ServerHook for GetRunRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let querys: RequestQuerys = ctx.get_request_querys().await;
        let id: i32 = match querys.get("id").and_then(|s: &String| s.parse().ok()) {
            Some(id) => id,
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Missing or invalid id parameter",
                );
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match CicdService::get_run_by_id(id).await {
            Ok(Some(run)) => {
                let response: ApiResponse<RunDto> = ApiResponse::success(run);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Ok(None) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::NotFound, "Run not found");
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
        };
    }
}

impl ServerHook for GetRunDetailRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let querys: RequestQuerys = ctx.get_request_querys().await;
        let id: i32 = match querys.get("id").and_then(|s: &String| s.parse().ok()) {
            Some(id) => id,
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Missing or invalid id parameter",
                );
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match CicdService::get_run_detail(id).await {
            Ok(Some(detail)) => {
                let response: ApiResponse<RunDetailDto> = ApiResponse::success(detail);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Ok(None) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::NotFound, "Run not found");
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
        };
    }
}

impl ServerHook for UpdateJobRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        request_body_json_result(param: UpdateJobStatusParam),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let param: UpdateJobStatusParam = match param {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match CicdService::update_job_status(param).await {
            Ok(()) => {
                let response: ApiResponse<()> =
                    ApiResponse::success_without_data("Job status updated successfully");
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
        };
    }
}

impl ServerHook for UpdateStepRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        request_body_json_result(param: UpdateStepStatusParam),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let param: UpdateStepStatusParam = match param {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match CicdService::update_step_status(param).await {
            Ok(()) => {
                let response: ApiResponse<()> =
                    ApiResponse::success_without_data("Step status updated successfully");
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
        };
    }
}

impl ServerHook for GetIncrementalRunDetailRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let querys: RequestQuerys = ctx.get_request_querys().await;
        let run_id: i32 = match querys.get("run_id").and_then(|s: &String| s.parse().ok()) {
            Some(id) => id,
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Missing or invalid run_id parameter",
                );
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };

        let step_offsets: Vec<StepOffsetParam> = match querys.get("offsets") {
            Some(offsets_str) => {
                serde_json::from_str::<Vec<StepOffsetParam>>(offsets_str).unwrap_or_default()
            }
            None => Vec::new(),
        };

        match CicdService::get_incremental_run_detail(run_id, step_offsets).await {
            Ok(Some(detail)) => {
                let response: ApiResponse<IncrementalRunDetailDto> = ApiResponse::success(detail);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Ok(None) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::NotFound, "Run not found");
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
        };
    }
}

impl ServerHook for CicdViewRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/cicd/index.html")
    )]
    #[instrument_trace]
    async fn handle(self, _ctx: &Context) {}
}

impl ServerHook for RunLogsSseRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => TEXT_EVENT_STREAM),
        response_header(CACHE_CONTROL => NO_CACHE),
        response_header(CONNECTION => KEEP_ALIVE)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let querys: RequestQuerys = ctx.get_request_querys().await;
        let run_id: i32 = match querys.get("run_id").and_then(|s: &String| s.parse().ok()) {
            Some(id) => id,
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Missing or invalid run_id parameter",
                );
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };

        ctx.send().await;

        let log_manager: &LogStreamManager = get_log_stream_manager();
        let step_ids: Vec<i32> = log_manager.get_run_step_ids(run_id).await;

        if step_ids.is_empty() {
            let completion_event: String = format!(
                "event: complete\ndata: {{\"run_id\":{run_id},\"reason\":\"no_active_streams\"}}{HTTP_DOUBLE_BR}"
            );
            ctx.set_response_body(&completion_event)
                .await
                .send_body()
                .await;
            ctx.closed().await;
            return;
        }
        let mut receivers: Vec<Receiver<LogEntry>> = Vec::new();
        for step_id in &step_ids {
            if let Some(receiver) = log_manager.subscribe_to_step(run_id, *step_id).await {
                receivers.push(receiver);
            }
        }
        if receivers.is_empty() {
            let completion_event: String = format!(
                "event: complete\ndata: {{\"run_id\":{run_id},\"reason\":\"no_active_streams\"}}{HTTP_DOUBLE_BR}"
            );
            ctx.set_response_body(&completion_event)
                .await
                .send_body()
                .await;
            ctx.closed().await;
            return;
        }

        let mut interval: tokio::time::Interval =
            tokio::time::interval(tokio::time::Duration::from_millis(100));
        let timeout_duration: tokio::time::Duration = tokio::time::Duration::from_secs(3600);
        let start_time: tokio::time::Instant = tokio::time::Instant::now();

        loop {
            if start_time.elapsed() > timeout_duration {
                let timeout_event: String = format!(
                    "event: complete\ndata: {{\"run_id\":{run_id},\"reason\":\"timeout\"}}{HTTP_DOUBLE_BR}"
                );
                ctx.set_response_body(&timeout_event)
                    .await
                    .send_body()
                    .await;
                break;
            }

            let mut has_activity: bool = false;

            for (idx, receiver) in receivers.iter_mut().enumerate() {
                let step_id: i32 = step_ids[idx];
                match receiver.try_recv() {
                    Ok(log_entry) => {
                        has_activity = true;
                        let log_event: String = format!(
                            "event: log\ndata: {{\"step_id\":{},\"timestamp\":{},\"is_stderr\":{},\"content\":\"{}\"}}{}",
                            step_id,
                            log_entry.timestamp,
                            log_entry.is_stderr,
                            Self::escape_json_string(&log_entry.content),
                            HTTP_DOUBLE_BR
                        );
                        ctx.set_response_body(&log_event).await.send_body().await;
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Lagged(_)) => {
                        has_activity = true;
                        let lag_event: String = format!(
                            "event: notice\ndata: {{\"step_id\":{step_id},\"message\":\"log_buffer_lagged\"}}{HTTP_DOUBLE_BR}"
                        );
                        ctx.set_response_body(&lag_event).await.send_body().await;
                    }
                    Err(TryRecvError::Closed) => {
                        continue;
                    }
                }
            }

            let current_step_ids: Vec<i32> = log_manager.get_run_step_ids(run_id).await;
            if current_step_ids.is_empty() && !has_activity {
                let completion_event: String = format!(
                    "event: complete\ndata: {{\"run_id\":{run_id},\"reason\":\"run_completed\"}}{HTTP_DOUBLE_BR}"
                );
                ctx.set_response_body(&completion_event)
                    .await
                    .send_body()
                    .await;
                break;
            }

            for step_id in &current_step_ids {
                if !step_ids.contains(step_id) {
                    if let Some(receiver) = log_manager.subscribe_to_step(run_id, *step_id).await {
                        receivers.push(receiver);
                    }
                }
            }

            if !has_activity {
                interval.tick().await;
            }
        }

        ctx.closed().await;
    }
}

impl RunLogsSseRoute {
    fn escape_json_string(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                '"' => "\\\"".to_string(),
                '\\' => "\\\\".to_string(),
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\t' => "\\t".to_string(),
                c if c.is_control() => format!("\\u{:04x}", c as u32),
                c => c.to_string(),
            })
            .collect()
    }
}
