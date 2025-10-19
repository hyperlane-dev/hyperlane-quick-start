use super::*;

impl ServerHook for ListRecordsRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        request_query("keys" => keys_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        let keys: Vec<String> = match keys_opt {
            Some(k) => k.split(',').map(|s: &str| s.to_string()).collect(),
            None => {
                let response = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Keys parameter is required",
                );
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match get_all_redis_records(keys).await {
            Ok(records) => {
                let response = ApiResponse::success(records);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Err(error) => {
                let response =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
        };
    }
}

impl ServerHook for CreateRecordRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        post,
        request_body_json(record_opt: RedisRecord),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        let record: RedisRecord = match record_opt {
            Ok(data) => data,
            Err(error) => {
                let response =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error.to_string());
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match create_redis_record(record).await {
            Ok(_) => {
                let response =
                    ApiResponse::<()>::success_without_data("Record created successfully");
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Err(error) => {
                let response =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
        };
    }
}

impl ServerHook for UpdateRecordRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        post,
        request_body_json(record_opt: RedisRecord),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        let record: RedisRecord = match record_opt {
            Ok(data) => data,
            Err(error) => {
                let response =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error.to_string());
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match update_redis_record(record).await {
            Ok(_) => {
                let response =
                    ApiResponse::<()>::success_without_data("Record updated successfully");
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Err(error) => {
                let response =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
        };
    }
}

impl ServerHook for DeleteRecordRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        post,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        let querys: RequestQuerys = ctx.get_request_querys().await;
        let key: &String = match querys.get("key") {
            Some(k) => k,
            None => {
                let response = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Key parameter is required",
                );
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match delete_redis_record(key).await {
            Ok(_) => {
                let response =
                    ApiResponse::<()>::success_without_data("Record deleted successfully");
                ctx.set_response_body(&response.to_json_bytes()).await
            }
            Err(error) => {
                let response =
                    ApiResponse::<()>::error_with_code(ResponseCode::DatabaseError, error);
                ctx.set_response_body(&response.to_json_bytes()).await
            }
        };
    }
}
