use super::*;

impl ServerHook for InsertRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        post,
        request_body_json_result(request_opt: ShortlinkInsertRequest),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        let request: ShortlinkInsertRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<()> =
                    ApiResponse::<()>::error_with_code(ResponseCode::BadRequest, error.to_string());
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match ShortlinkService::insert_shortlink(request).await {
            Ok(id) => {
                let response: ApiResponse<i32> = ApiResponse::<i32>::success(id);
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
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        route_param_option(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    async fn handle(self, ctx: &Context) {
        let id: i32 = match id_opt {
            Some(id_str) => match id_str.parse::<i32>() {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                        ResponseCode::BadRequest,
                        "Invalid ID parameter",
                    );
                    ctx.set_response_body(&response.to_json_bytes()).await;
                    return;
                }
            },
            None => {
                let response: ApiResponse<()> = ApiResponse::<()>::error_with_code(
                    ResponseCode::BadRequest,
                    "ID parameter is required",
                );
                ctx.set_response_body(&response.to_json_bytes()).await;
                return;
            }
        };
        match ShortlinkService::query_shortlink(id).await {
            Ok(Some(record)) => {
                let response: ApiResponse<ShortlinkRecord> =
                    ApiResponse::<ShortlinkRecord>::success(record);
                ctx.set_response_status_code(302)
                    .await
                    .set_response_header(
                        LOCATION,
                        response.get_data().clone().unwrap_or_default().get_url(),
                    )
                    .await
            }
            Ok(None) => {
                let response = ApiResponse::<()>::error_with_code(
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
