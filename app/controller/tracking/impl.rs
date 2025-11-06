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
        get,
        response_status_code(200),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        #[request_query("start_time" => start_time_opt)]
        async fn get_start_time(ctx: &Context) -> Option<String> {
            start_time_opt
        }

        #[request_query("end_time" => end_time_opt)]
        async fn get_end_time(ctx: &Context) -> Option<String> {
            end_time_opt
        }

        #[request_query("socket_addr" => socket_addr_opt)]
        async fn get_socket_addr(ctx: &Context) -> Option<String> {
            socket_addr_opt
        }

        #[request_query("header_key" => header_key_opt)]
        async fn get_header_key(ctx: &Context) -> Option<String> {
            header_key_opt
        }

        #[request_query("header_value" => header_value_opt)]
        async fn get_header_value(ctx: &Context) -> Option<String> {
            header_value_opt
        }

        #[request_query("body_content" => body_content_opt)]
        async fn get_body_content(ctx: &Context) -> Option<String> {
            body_content_opt
        }

        #[request_query("page" => page_opt)]
        async fn get_page(ctx: &Context) -> Option<String> {
            page_opt
        }

        #[request_query("page_size" => page_size_opt)]
        async fn get_page_size(ctx: &Context) -> Option<String> {
            page_size_opt
        }
        let start_time: Option<i64> = get_start_time(ctx)
            .await
            .and_then(|s| s.parse::<i64>().ok());
        let end_time: Option<i64> = get_end_time(ctx).await.and_then(|s| s.parse::<i64>().ok());
        let socket_addr: Option<String> = get_socket_addr(ctx).await;
        let header_key: Option<String> = get_header_key(ctx).await;
        let header_value: Option<String> = get_header_value(ctx).await;
        let body_content: Option<String> = get_body_content(ctx).await;
        let page: Option<i64> = get_page(ctx).await.and_then(|s| s.parse::<i64>().ok());
        let page_size: Option<i64> = get_page_size(ctx).await.and_then(|s| s.parse::<i64>().ok());
        let request: TrackingQueryRequest = TrackingQueryRequest {
            start_time,
            end_time,
            socket_addr,
            header_key,
            header_value,
            body_content,
            page,
            page_size,
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
