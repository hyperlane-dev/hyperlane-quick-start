use super::*;

impl ServerHook for ListRecordsRoute {
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
        match RedisService::get_all_redis_records().await {
            Ok(records) => {
                let response: ApiResponse<Vec<RedisRecord>> =
                    ApiResponse::new(ApiResponseStatus::Success, records);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

impl ServerHook for CreateRecordRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        request_body_json_result(record_opt: RedisRecord),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let record: RedisRecord = match record_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match RedisService::create_redis_record(record).await {
            Ok(_) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Success, SUCCESS_RECORD_CREATED);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

impl ServerHook for UpdateRecordRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        request_body_json_result(record_opt: RedisRecord),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let record: RedisRecord = match record_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match RedisService::update_redis_record(record).await {
            Ok(_) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Success, SUCCESS_RECORD_UPDATED);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}

impl ServerHook for DeleteRecordRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let querys: &RequestQuerys = ctx.get_request().get_querys();
        let key: &String = match querys.get("key") {
            Some(k) => k,
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    ERROR_KEY_PARAMETER_REQUIRED,
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match RedisService::delete_redis_record(key).await {
            Ok(_) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Success, SUCCESS_RECORD_DELETED);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::DatabaseError, error);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
        };
        Status::Continue
    }
}
