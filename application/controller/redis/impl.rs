use super::*;

/// Implementation of `ListRecordsRoute` for `ServerHook`.
impl ServerHook for ListRecordsRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `ListRecordsRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    /// Handles the `ListRecordsRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
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

/// Implementation of `CreateRecordRoute` for `ServerHook`.
impl ServerHook for CreateRecordRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `CreateRecordRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        request_body_json_result(record_opt: RedisRecord),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    /// Handles the `CreateRecordRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
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

/// Implementation of `UpdateRecordRoute` for `ServerHook`.
impl ServerHook for UpdateRecordRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `UpdateRecordRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        request_body_json_result(record_opt: RedisRecord),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    /// Handles the `UpdateRecordRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
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

/// Implementation of `DeleteRecordRoute` for `ServerHook`.
impl ServerHook for DeleteRecordRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `DeleteRecordRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    /// Handles the `DeleteRecordRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
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
