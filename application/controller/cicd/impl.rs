use super::*;

impl ServerHook for CreatePipelineRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        request_body_json_result(param: CreatePipelineParam),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let param: CreatePipelineParam = match param {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match CicdService::create_pipeline(param).await {
            Ok(id) => {
                let response: ApiResponse<i32> = ApiResponse::success(id);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for ListPipelinesRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        match CicdService::get_all_pipelines().await {
            Ok(pipelines) => {
                let response: ApiResponse<Vec<PipelineDto>> = ApiResponse::success(pipelines);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for GetPipelineRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let id: i32 = match ctx
            .get_request()
            .get_querys()
            .get("id")
            .and_then(|s: &String| s.parse().ok())
        {
            Some(id) => id,
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Missing or invalid id parameter",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match CicdService::get_pipeline_by_id(id).await {
            Ok(Some(pipeline)) => {
                let response: ApiResponse<PipelineDto> = ApiResponse::success(pipeline);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::NotFound,
                    "Pipeline not found",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for TriggerRunRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        request_body_json_result(param: TriggerRunParam),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let param: TriggerRunParam = match param {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match CicdService::trigger_run(param).await {
            Ok(id) => {
                let response: ApiResponse<i32> = ApiResponse::success(id);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for ListRunsRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let querys: &RequestQuerys = ctx.get_request().get_querys();
        let page_size: Option<i32> = querys
            .get("page_size")
            .and_then(|s: &String| s.parse().ok());
        let last_id: Option<i32> = querys.get("last_id").and_then(|s: &String| s.parse().ok());
        let pipeline_id: Option<i32> = querys
            .get("pipeline_id")
            .and_then(|s: &String| s.parse().ok());
        let mut param = QueryRunsParam::default();
        param
            .set_pipeline_id(pipeline_id)
            .set_status(None)
            .set_page_size(page_size)
            .set_last_id(last_id);
        match CicdService::query_runs(param).await {
            Ok(result) => {
                let response: ApiResponse<PaginatedRunsDto> = ApiResponse::success(result);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for GetRunRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let querys: &RequestQuerys = ctx.get_request().get_querys();
        let id: i32 = match querys.get("id").and_then(|s: &String| s.parse().ok()) {
            Some(id) => id,
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Missing or invalid id parameter",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match CicdService::get_run_by_id(id).await {
            Ok(Some(run)) => {
                let response: ApiResponse<RunDto> = ApiResponse::success(run);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::NotFound, "Run not found");
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for GetRunDetailRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let id: i32 = match ctx
            .get_request()
            .get_querys()
            .get("id")
            .and_then(|s: &String| s.parse().ok())
        {
            Some(id) => id,
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Missing or invalid id parameter",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match CicdService::get_run_detail(id).await {
            Ok(Some(detail)) => {
                let response: ApiResponse<RunDetailDto> = ApiResponse::success(detail);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::NotFound, "Run not found");
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for UpdateJobRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        request_body_json_result(param: UpdateJobStatusParam),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let param: UpdateJobStatusParam = match param {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match CicdService::update_job_status(param).await {
            Ok(()) => {
                let response: ApiResponse<()> =
                    ApiResponse::success_without_data("Job status updated successfully");
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for UpdateStepRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        request_body_json_result(param: UpdateStepStatusParam),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let param: UpdateStepStatusParam = match param {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        match CicdService::update_step_status(param).await {
            Ok(()) => {
                let response: ApiResponse<()> =
                    ApiResponse::success_without_data("Step status updated successfully");
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for GetIncrementalRunDetailRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let querys: &RequestQuerys = ctx.get_request().get_querys();
        let run_id: i32 = match querys.get("run_id").and_then(|s: &String| s.parse().ok()) {
            Some(id) => id,
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Missing or invalid run_id parameter",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
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
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::NotFound, "Run not found");
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
    }
}

impl ServerHook for CicdViewRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/cicd/index.html")
    )]
    #[instrument_trace]
    async fn handle(self, _ctx: &mut Context) {}
}

impl ServerHook for RunLogsSseRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => TEXT_EVENT_STREAM),
        response_header(CACHE_CONTROL => NO_CACHE),
        response_header(CONNECTION => KEEP_ALIVE)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let querys: &RequestQuerys = ctx.get_request().get_querys();
        let run_id: i32 = match querys.get("run_id").and_then(|s: &String| s.parse().ok()) {
            Some(id) => id,
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Missing or invalid run_id parameter",
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return;
            }
        };
        ctx.send().await;
        let log_manager: &LogStreamManager = get_log_stream_manager();
        let mut step_ids: Vec<i32> = log_manager.get_run_step_ids(run_id).await;
        if step_ids.is_empty() {
            sleep(Duration::from_millis(500)).await;
            step_ids = log_manager.get_run_step_ids(run_id).await;
            if step_ids.is_empty() {
                let completion_event: String = format!(
                    "event: complete\ndata: {{\"run_id\":{run_id},\"reason\":\"no_active_streams\"}}{HTTP_DOUBLE_BR}"
                );
                ctx.get_mut_response().set_body(&completion_event);
                ctx.send_body().await;
                ctx.set_closed(true);
                return;
            }
        }
        let mut receivers: Vec<(i32, BroadcastMapReceiver<String>)> = Vec::new();
        for step_id in &step_ids {
            if let Some(receiver) = log_manager.create_step_receiver(run_id, *step_id).await {
                receivers.push((*step_id, receiver));
            }
        }
        if receivers.is_empty() {
            let completion_event: String = format!(
                "event: complete\ndata: {{\"run_id\":{run_id},\"reason\":\"no_active_streams\"}}{HTTP_DOUBLE_BR}"
            );
            ctx.get_mut_response().set_body(&completion_event);
            ctx.send_body().await;
            ctx.set_closed(true);
            return;
        }
        let timeout_duration: Duration = Duration::from_secs(3600);
        let start_time: Instant = Instant::now();
        loop {
            if start_time.elapsed() > timeout_duration {
                let timeout_event: String = format!(
                    "event: complete\ndata: {{\"run_id\":{run_id},\"reason\":\"timeout\"}}{HTTP_DOUBLE_BR}"
                );
                ctx.get_mut_response().set_body(&timeout_event);
                ctx.send_body().await;
                break;
            }
            let mut has_activity: bool = false;
            for (step_id, receiver) in receivers.iter_mut() {
                while let Ok(entry_json) = receiver.try_recv() {
                    if let Ok(entry) = serde_json::from_str::<LogEntry>(&entry_json) {
                        has_activity = true;
                        let log_event: String = format!(
                            "event: log\ndata: {{\"step_id\":{},\"timestamp\":{},\"is_stderr\":{},\"content\":\"{}\"}}{}",
                            step_id,
                            entry.get_timestamp(),
                            entry.get_is_stderr(),
                            Self::escape_json_string(entry.get_content()),
                            HTTP_DOUBLE_BR
                        );
                        ctx.get_mut_response().set_body(&log_event);
                        ctx.send_body().await;
                    }
                }
            }
            let current_step_ids: Vec<i32> = log_manager.get_run_step_ids(run_id).await;
            if current_step_ids.is_empty() && !has_activity {
                let completion_event: String = format!(
                    "event: complete\ndata: {{\"run_id\":{run_id},\"reason\":\"run_completed\"}}{HTTP_DOUBLE_BR}"
                );
                ctx.get_mut_response().set_body(&completion_event);
                ctx.send_body().await;
                break;
            }
            for step_id in &current_step_ids {
                if !step_ids.contains(step_id) {
                    if let Some(receiver) = log_manager.create_step_receiver(run_id, *step_id).await
                    {
                        receivers.push((*step_id, receiver));
                        step_ids.push(*step_id);
                    }
                }
            }
            if !has_activity {
                sleep(Duration::from_millis(10)).await;
            }
        }
        ctx.set_closed(true);
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
