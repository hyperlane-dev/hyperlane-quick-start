use super::*;

impl ServerHook for TrackingReportRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        post,
        response_status_code(200),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        let body: RequestBody = ctx.get_request_body().await;
        match TrackingService::save_tracking_record(ctx, &body).await {
            Ok(_) => {
                let response: ApiResponse<Vec<u8>> = ApiResponse::default_success();
                ctx.set_response_body(&response.to_json_bytes()).await;
            }
            Err(error) => {
                let error_response: ApiResponse<()> = ApiResponse::error(&error);
                ctx.set_response_body(&error_response.to_json_bytes()).await;
            }
        }
    }
}

impl ServerHook for TrackingQueryRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        post,
        response_status_code(200),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        let body: RequestBody = ctx.get_request_body().await;
        let request: TrackingQueryRequest = match serde_json::from_slice(&body) {
            Ok(req) => req,
            Err(error) => {
                let error_response: ApiResponse<()> =
                    ApiResponse::error(&format!("Invalid request body: {error}"));
                ctx.set_response_body(&error_response.to_json_bytes()).await;
                return;
            }
        };
        match TrackingService::query_tracking_records(request).await {
            Ok(result) => {
                let response: ApiResponse<TrackingQueryResponse> = ApiResponse::success(result);
                ctx.set_response_body(&response.to_json_bytes()).await;
            }
            Err(error) => {
                let error_response: ApiResponse<()> = ApiResponse::error(&error);
                ctx.set_response_body(&error_response.to_json_bytes()).await;
            }
        }
    }
}
