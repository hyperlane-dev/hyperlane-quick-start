use super::*;

/// Implementation of `InsertRoute` for `ServerHook`.
impl ServerHook for InsertRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        request_body_json_result(request_opt: ShortlinkInsertRequest),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let request: ShortlinkInsertRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match ShortlinkService::insert_shortlink(request).await {
            Ok(encrypted_id) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Success, encrypted_id);
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

/// Implementation of `QueryRoute` for `ServerHook`.
impl ServerHook for QueryRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        try_get_route_param(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let encrypted_id: String = match id_opt {
            Some(id_str) => id_str,
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    ERROR_SHORTLINK_ID_REQUIRED,
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match ShortlinkService::query_shortlink(encrypted_id).await {
            Ok(Some(record)) => {
                let response: ApiResponse<ShortlinkRecord> =
                    ApiResponse::new(ApiResponseStatus::Success, record);
                ctx.get_mut_response().set_status_code(302).set_header(
                    LOCATION,
                    response
                        .try_get_data()
                        .clone()
                        .unwrap_or_default()
                        .get_url(),
                )
            }
            Ok(None) => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::ResourceNotFound,
                    ERROR_SHORTLINK_NOT_FOUND,
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
