use super::*;

impl ServerHook for ListRecordsRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        match PostgresqlService::get_all_postgresql_records().await {
            Ok(records) => {
                let response: ApiResponse<Vec<PostgresqlRecord>> = ApiResponse::success(records);
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
        request_body_json(record_opt: PostgresqlRecord),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        let record: PostgresqlRecord = match record_opt {
            Ok(data) => data,
            Err(error) => {
                let response =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error.to_string());
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match PostgresqlService::create_postgresql_record(record).await {
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
        request_body_json(record_opt: PostgresqlRecord),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        let record: PostgresqlRecord = match record_opt {
            Ok(data) => data,
            Err(error) => {
                let response =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error.to_string());
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match PostgresqlService::update_postgresql_record(record).await {
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
        let key: String = match ctx.get_request_querys().await.get("key").cloned() {
            Some(k) => k,
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "Key parameter is required",
                );
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match PostgresqlService::delete_postgresql_record(&key).await {
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
