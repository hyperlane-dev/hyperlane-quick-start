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

impl ServerHook for UpdatePipelineRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        request_body_json_result(param: UpdatePipelineParam),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let param: UpdatePipelineParam = match param {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match CicdService::update_pipeline(param).await {
            Ok(()) => {
                let response: ApiResponse<()> =
                    ApiResponse::success_without_data("Pipeline updated successfully");
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

impl ServerHook for DeletePipelineRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
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
        match CicdService::delete_pipeline(id).await {
            Ok(()) => {
                let response: ApiResponse<()> =
                    ApiResponse::success_without_data("Pipeline deleted successfully");
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
        let param: QueryRunsParam = QueryRunsParam {
            pipeline_id: None,
            status: None,
            page: Some(1),
            page_size: Some(50),
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
