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
                let response: ApiResponse<()> = ApiResponse::new(ApiResponseStatus::Success, ());
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
            Err(error) => {
                let error_response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InternalServerError, error);
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
                let error_response: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    format!("Invalid request body {error}"),
                );
                ctx.get_mut_response()
                    .set_body(error_response.to_json_bytes());
                return;
            }
        };
        match TrackingService::query_tracking_records(request).await {
            Ok(result) => {
                let response: ApiResponse<TrackingQueryResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, result);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
            Err(error) => {
                let error_response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InternalServerError, error);
                ctx.get_mut_response()
                    .set_body(error_response.to_json_bytes());
            }
        }
    }
}
