use super::*;

/// Implementation of `CreatePipelineRoute` for `ServerHook`.
impl ServerHook for CreatePipelineRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        request_body_json_result(param: CreatePipelineParam),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let param: CreatePipelineParam = match param {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match CicdService::create_pipeline(param).await {
            Ok(id) => {
                let response: ApiResponse<i32> = ApiResponse::new(ApiResponseStatus::Success, id);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `ListPipelinesRoute` for `ServerHook`.
impl ServerHook for ListPipelinesRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        match CicdService::get_all_pipelines().await {
            Ok(pipelines) => {
                let response: ApiResponse<Vec<PipelineDto>> =
                    ApiResponse::new(ApiResponseStatus::Success, pipelines);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `GetPipelineRoute` for `ServerHook`.
impl ServerHook for GetPipelineRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let id: i32 = match ctx
            .get_request()
            .get_querys()
            .get("id")
            .and_then(|s: &String| s.parse().ok())
        {
            Some(id) => id,
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    ERROR_MISSING_OR_INVALID_ID,
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match CicdService::get_pipeline_by_id(id).await {
            Ok(Some(pipeline)) => {
                let response: ApiResponse<PipelineDto> =
                    ApiResponse::new(ApiResponseStatus::Success, pipeline);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::ResourceNotFound,
                    ERROR_PIPELINE_NOT_FOUND,
                );
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `TriggerRunRoute` for `ServerHook`.
impl ServerHook for TriggerRunRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        request_body_json_result(param: TriggerRunParam),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let param: TriggerRunParam = match param {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match CicdService::trigger_run(param).await {
            Ok(id) => {
                let response: ApiResponse<i32> = ApiResponse::new(ApiResponseStatus::Success, id);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `ListRunsRoute` for `ServerHook`.
impl ServerHook for ListRunsRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let querys: &RequestQuerys = ctx.get_request().get_querys();
        let page_size: Option<i32> = querys
            .get("page_size")
            .and_then(|s: &String| s.parse().ok())
            .map(|p: i32| p.min(MAX_PAGE_SIZE));
        let last_id: Option<i32> = querys.get("last_id").and_then(|s: &String| s.parse().ok());
        let pipeline_id: Option<i32> = querys
            .get("pipeline_id")
            .and_then(|s: &String| s.parse().ok());
        let mut param: QueryRunsParam = QueryRunsParam::default();
        param
            .set_pipeline_id(pipeline_id)
            .set_status(None)
            .set_page_size(page_size)
            .set_last_id(last_id);
        match CicdService::query_runs(param).await {
            Ok(result) => {
                let response: ApiResponse<PaginatedRunsDto> =
                    ApiResponse::new(ApiResponseStatus::Success, result);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `GetRunRoute` for `ServerHook`.
impl ServerHook for GetRunRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let querys: &RequestQuerys = ctx.get_request().get_querys();
        let id: i32 = match querys.get("id").and_then(|s: &String| s.parse().ok()) {
            Some(id) => id,
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    ERROR_MISSING_OR_INVALID_ID,
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match CicdService::get_run_by_id(id).await {
            Ok(Some(run)) => {
                let response: ApiResponse<RunDto> =
                    ApiResponse::new(ApiResponseStatus::Success, run);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::ResourceNotFound, ERROR_RUN_NOT_FOUND);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `GetRunDetailRoute` for `ServerHook`.
impl ServerHook for GetRunDetailRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let id: i32 = match ctx
            .get_request()
            .get_querys()
            .get("id")
            .and_then(|s: &String| s.parse().ok())
        {
            Some(id) => id,
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    ERROR_MISSING_OR_INVALID_ID,
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match CicdService::get_run_detail(id).await {
            Ok(Some(detail)) => {
                let response: ApiResponse<RunDetailDto> =
                    ApiResponse::new(ApiResponseStatus::Success, detail);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::ResourceNotFound, ERROR_RUN_NOT_FOUND);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `UpdateJobRoute` for `ServerHook`.
impl ServerHook for UpdateJobRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        request_body_json_result(param: UpdateJobStatusParam),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let param: UpdateJobStatusParam = match param {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match CicdService::update_job_status(param).await {
            Ok(()) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Success, SUCCESS_JOB_STATUS_UPDATED);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `UpdateStepRoute` for `ServerHook`.
impl ServerHook for UpdateStepRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        request_body_json_result(param: UpdateStepStatusParam),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let param: UpdateStepStatusParam = match param {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match CicdService::update_step_status(param).await {
            Ok(()) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Success, SUCCESS_STEP_STATUS_UPDATED);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `GetIncrementalRunDetailRoute` for `ServerHook`.
impl ServerHook for GetIncrementalRunDetailRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let querys: &RequestQuerys = ctx.get_request().get_querys();
        let run_id: i32 = match querys.get("run_id").and_then(|s: &String| s.parse().ok()) {
            Some(id) => id,
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    ERROR_MISSING_OR_INVALID_RUN_ID,
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let step_offsets: Vec<StepOffsetParam> = match querys.get("offsets") {
            Some(offsets_str) => {
                serde_json::from_str::<Vec<StepOffsetParam>>(offsets_str).unwrap_or_default()
            }
            None => vec![],
        };
        match CicdService::get_incremental_run_detail(run_id, step_offsets).await {
            Ok(Some(detail)) => {
                let response: ApiResponse<IncrementalRunDetailDto> =
                    ApiResponse::new(ApiResponseStatus::Success, detail);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::ResourceNotFound, ERROR_RUN_NOT_FOUND);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

/// Implementation of `CicdViewRoute` for `ServerHook`.
impl ServerHook for CicdViewRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => CICD_VIEW_REDIRECT_PATH)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

/// Implementation of `RunLogsSseRoute` for `ServerHook`.
impl ServerHook for RunLogsSseRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => TEXT_EVENT_STREAM),
        response_header(CACHE_CONTROL => NO_CACHE),
        response_header(CONNECTION => KEEP_ALIVE)
    )]
    #[instrument_trace]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let querys: &RequestQuerys = ctx.get_request().get_querys();
        let run_id: i32 = match querys.get("run_id").and_then(|s: &String| s.parse().ok()) {
            Some(id) => id,
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    ERROR_MISSING_OR_INVALID_RUN_ID,
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let _: Result<(), ResponseError> = stream.try_send(ctx.get_mut_response().build()).await;
        let log_manager: &LogStreamManager = get_log_stream_manager();
        let mut step_ids: Vec<i32> = log_manager.get_run_step_ids(run_id).await;
        if step_ids.is_empty() {
            sleep(Duration::from_millis(SSE_INITIAL_POLL_DELAY_MS)).await;
            step_ids = log_manager.get_run_step_ids(run_id).await;
            if step_ids.is_empty() {
                let completion_event: String = format!(
                    "event: {SSE_EVENT_COMPLETE}{BR}data: {{\"run_id\":{run_id},\"reason\":\"{SSE_REASON_NO_ACTIVE_STREAMS}\"}}{HTTP_DOUBLE_BR}"
                );
                ctx.get_mut_response().set_body(&completion_event);
                let _: Result<(), ResponseError> =
                    stream.try_send(ctx.get_mut_response().build()).await;
                stream.set_closed(true);
                return Status::Reject;
            }
        }
        let mut receivers: Vec<(i32, BroadcastMapReceiver<String>)> = vec![];
        for step_id in &step_ids {
            if let Some(receiver) = log_manager.create_step_receiver(run_id, *step_id).await {
                receivers.push((*step_id, receiver));
            }
        }
        if receivers.is_empty() {
            let completion_event: String = format!(
                "event: {SSE_EVENT_COMPLETE}{BR}data: {{\"run_id\":{run_id},\"reason\":\"{SSE_REASON_NO_ACTIVE_STREAMS}\"}}{HTTP_DOUBLE_BR}"
            );
            ctx.get_mut_response().set_body(&completion_event);
            let _: Result<(), ResponseError> =
                stream.try_send(ctx.get_mut_response().build()).await;
            stream.set_closed(true);
            return Status::Reject;
        }
        let timeout_duration: Duration = Duration::from_secs(SSE_CONNECTION_TIMEOUT_SECS);
        let start_time: Instant = Instant::now();
        loop {
            if start_time.elapsed() > timeout_duration {
                let timeout_event: String = format!(
                    "event: {SSE_EVENT_COMPLETE}{BR}data: {{\"run_id\":{run_id},\"reason\":\"{SSE_REASON_TIMEOUT}\"}}{HTTP_DOUBLE_BR}"
                );
                ctx.get_mut_response().set_body(&timeout_event);
                let _: Result<(), ResponseError> =
                    stream.try_send(ctx.get_mut_response().build()).await;
                break;
            }
            let mut has_activity: bool = false;
            for (step_id, receiver) in receivers.iter_mut() {
                while let Ok(entry_json) = receiver.try_recv() {
                    if let Ok(entry) = serde_json::from_str::<LogEntry>(&entry_json) {
                        has_activity = true;
                        let log_event: String = format!(
                            "event: {SSE_EVENT_LOG}{BR}data: {{\"step_id\":{},\"timestamp\":{},\"is_stderr\":{},\"content\":\"{}\"}}{}",
                            step_id,
                            entry.get_timestamp(),
                            entry.get_is_stderr(),
                            Self::escape_json_string(entry.get_content()),
                            HTTP_DOUBLE_BR
                        );
                        ctx.get_mut_response().set_body(&log_event);
                        let _: Result<(), ResponseError> =
                            stream.try_send(ctx.get_mut_response().build()).await;
                    }
                }
            }
            let current_step_ids: Vec<i32> = log_manager.get_run_step_ids(run_id).await;
            if current_step_ids.is_empty() && !has_activity {
                let completion_event: String = format!(
                    "event: {SSE_EVENT_COMPLETE}{BR}data: {{\"run_id\":{run_id},\"reason\":\"{SSE_REASON_RUN_COMPLETED}\"}}{HTTP_DOUBLE_BR}"
                );
                ctx.get_mut_response().set_body(&completion_event);
                let _: Result<(), ResponseError> =
                    stream.try_send(ctx.get_mut_response().build()).await;
                break;
            }
            for step_id in &current_step_ids {
                if !step_ids.contains(step_id)
                    && let Some(receiver) = log_manager.create_step_receiver(run_id, *step_id).await
                {
                    receivers.push((*step_id, receiver));
                    step_ids.push(*step_id);
                }
            }
            if !has_activity {
                sleep(Duration::from_millis(SSE_IDLE_SLEEP_MS)).await;
            }
        }
        stream.set_closed(true);
        Status::Reject
    }
}

/// Implementation of methods for `RunLogsSseRoute`.
impl RunLogsSseRoute {
    fn escape_json_string(escape: &str) -> String {
        escape
            .chars()
            .map(|escape_item| match escape_item {
                '"' => "\\\"".to_string(),
                '\\' => "\\\\".to_string(),
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\t' => "\\t".to_string(),
                escape_item if escape_item.is_control() => format!("\\u{:04x}", escape_item as u32),
                escape_item => escape_item.to_string(),
            })
            .collect()
    }
}
