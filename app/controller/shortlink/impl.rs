use super::*;

impl ServerHook for InsertRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        post,
        request_body_json_result(request_opt: ShortlinkInsertRequest),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let request: ShortlinkInsertRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error);
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match ShortlinkService::insert_shortlink(request).await {
            Ok(encrypted_id) => {
                let response: ApiResponse<String> = ApiResponse::<String>::success(encrypted_id);
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

impl ServerHook for QueryRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        route_param_option(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let encrypted_id: String = match id_opt {
            Some(id_str) => id_str,
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Shortlink ID parameter is required",
                );
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match ShortlinkService::query_shortlink(encrypted_id).await {
            Ok(Some(record)) => {
                let response: ApiResponse<ShortlinkRecord> =
                    ApiResponse::<ShortlinkRecord>::success(record);
                ctx.set_response_status_code(302)
                    .await
                    .set_response_header(
                        LOCATION,
                        response
                            .try_get_data()
                            .clone()
                            .unwrap_or_default()
                            .get_url(),
                    )
                    .await
            }
            Ok(None) => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::NotFound,
                    "Shortlink not found",
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
