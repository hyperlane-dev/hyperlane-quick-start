use super::*;

impl ServerHook for TrackingReportRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let body: RequestBody = ctx.get_request().get_body().clone();
        match TrackingService::save_tracking_record(ctx, &body).await {
            Ok(_) => {
                let response: ApiResponse<()> = ApiResponse::default_success();
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
            Err(error) => {
                let error_response: ApiResponse<()> = ApiResponse::error(&error);
                ctx.get_mut_response()
                    .set_body(error_response.to_json_bytes());
            }
        }
    }
}

impl ServerHook for TrackingQueryRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        post_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let body: &RequestBody = ctx.get_request().get_body();
        let request: TrackingQueryRequest = match serde_json::from_slice(body) {
            Ok(req) => req,
            Err(error) => {
                let error_response: ApiResponse<()> =
                    ApiResponse::error(format!("Invalid request body {error}"));
                ctx.get_mut_response()
                    .set_body(error_response.to_json_bytes());
                return;
            }
        };
        match TrackingService::query_tracking_records(request).await {
            Ok(result) => {
                let response: ApiResponse<TrackingQueryResponse> = ApiResponse::success(result);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
            Err(error) => {
                let error_response: ApiResponse<()> = ApiResponse::error(&error);
                ctx.get_mut_response()
                    .set_body(error_response.to_json_bytes());
            }
        }
    }
}
