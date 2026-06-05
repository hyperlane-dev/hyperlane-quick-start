use super::*;

impl ServerHook for RecordCreateRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_post_method, request_body_json_result(request_opt: CreateRecordRequest), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let request: CreateRecordRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let current_user: UserResponse = match AuthService::get_user(current_user_id).await {
            Ok(Some(user_info)) => user_info,
            Ok(None) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, ERROR_USER_NOT_FOUND);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
        if !user_role.is_admin() {
            let response: ApiResponse<&str> = ApiResponse::new(
                ApiResponseStatus::Forbidden,
                ERROR_ONLY_ADMIN_CAN_CREATE_RECORDS,
            );
            ctx.get_mut_response().set_body(response.to_json_bytes());
            return Status::Continue;
        }
        let target_user_id: i32 = match request.try_get_target_user_id() {
            Some(target_id) => {
                let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
                if !user_role.is_admin() {
                    let response: ApiResponse<&str> = ApiResponse::new(
                        ApiResponseStatus::Forbidden,
                        ERROR_ONLY_ADMIN_CAN_CREATE_FOR_OTHERS,
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
                match AuthService::decode_id(target_id) {
                    Ok(decoded_id) => decoded_id,
                    Err(error) => {
                        let response: ApiResponse<String> =
                            ApiResponse::new(ApiResponseStatus::InvalidRequest, error);
                        ctx.get_mut_response().set_body(response.to_json_bytes());
                        return Status::Continue;
                    }
                }
            }
            None => current_user_id,
        };
        match OrderService::create_record(target_user_id, request).await {
            Ok(result) => {
                let response: ApiResponse<CreateRecordWithImagesResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, result);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
        }
        Status::Continue
    }
}

impl ServerHook for RecordListRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_get_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    #[try_get_request_query("user_id" => user_id_opt)]
    #[try_get_request_query("start_date" => start_date_opt)]
    #[try_get_request_query("end_date" => end_date_opt)]
    #[try_get_request_query("category" => category_opt)]
    #[try_get_request_query("transaction_type" => transaction_type_opt)]
    #[try_get_request_query("cache_id" => cache_id_opt)]
    #[try_get_request_query("page" => page_opt)]
    #[try_get_request_query("limit" => limit_opt)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let current_user: UserResponse = match AuthService::get_user(current_user_id).await {
            Ok(Some(user_info)) => user_info,
            Ok(None) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, ERROR_USER_NOT_FOUND);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let mut query: RecordQueryRequest = RecordQueryRequest::default();
        let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
        if user_role.is_admin() {
            if let Some(user_id_str) = user_id_opt
                && let Ok(user_id) = AuthService::decode_id(&user_id_str)
            {
                query.set_user_id(Some(user_id));
            }
        } else {
            query.set_user_id(Some(current_user_id));
        }
        if let Some(start_date_str) = start_date_opt
            && let Ok(start_date) = NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d")
        {
            query.set_start_date(Some(start_date));
        }
        if let Some(end_date_str) = end_date_opt
            && let Ok(end_date) = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d")
        {
            query.set_end_date(Some(end_date));
        }
        if let Some(category) = category_opt {
            query.set_category(Some(category));
        }
        if let Some(transaction_type) = transaction_type_opt {
            query.set_transaction_type(Some(transaction_type));
        }
        if let Some(cache_id_str) = cache_id_opt
            && let Ok(cache_id) = cache_id_str.parse::<i32>()
        {
            query.set_cache_id(Some(cache_id));
        }
        if let Some(page_str) = page_opt
            && let Ok(page) = page_str.parse::<i32>()
        {
            query.set_page(Some(page));
        }
        if let Some(limit_str) = limit_opt
            && let Ok(limit) = limit_str.parse::<u64>()
        {
            query.set_limit(Some(limit.min(MAX_LIMIT)));
        }
        match OrderService::list_records(query).await {
            Ok(list_response) => {
                let response: ApiResponse<RecordListResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, list_response);
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

impl ServerHook for RecordGetRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_get_method, try_get_route_param(ID_KEY => id_opt), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let record_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> = ApiResponse::new(
                        ApiResponseStatus::InvalidRequest,
                        ERROR_INVALID_RECORD_ID,
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_RECORD_ID_REQUIRED);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match OrderService::get_record(record_id).await {
            Ok(Some(record)) => {
                let response: ApiResponse<RecordResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, record);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::ResourceNotFound, ERROR_RECORD_NOT_FOUND);
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

impl ServerHook for OverviewStatisticsRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_get_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let current_user: UserResponse = match AuthService::get_user(current_user_id).await {
            Ok(Some(user_info)) => user_info,
            Ok(None) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, ERROR_USER_NOT_FOUND);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let user_role: UserRole = current_user.get_role().parse().unwrap_or_default();
        if !user_role.is_admin() {
            let response: ApiResponse<&str> =
                ApiResponse::new(ApiResponseStatus::Forbidden, ERROR_ONLY_ADMIN_OVERVIEW);
            ctx.get_mut_response().set_body(response.to_json_bytes());
            return Status::Continue;
        }
        match OrderService::get_overview_statistics().await {
            Ok(statistics) => {
                let response: ApiResponse<OverviewStatisticsResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, statistics);
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

impl ServerHook for ImageUploadRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_post_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[try_get_request_header(X_FILE_NAME => file_name_opt)]
    #[try_get_request_header(X_ORIGINAL_NAME => original_name_opt)]
    #[try_get_request_header(X_MIME_TYPE => mime_type_opt)]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => id,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let file_name: String = match file_name_opt {
            Some(s) => urlencoding::decode(&s).unwrap_or_default().to_string(),
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_MISSING_X_FILE_NAME);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let mime_type: String = match mime_type_opt {
            Some(s) => s,
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_MISSING_X_MIME_TYPE);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let file_data: Vec<u8> = ctx.get_request().get_body().clone();
        let original_name: Option<String> = original_name_opt
            .map(|s: String| urlencoding::decode(&s).unwrap_or_default().to_string());
        match OrderService::upload_image(
            current_user_id,
            file_name,
            original_name,
            mime_type,
            file_data,
        )
        .await
        {
            Ok(image_response) => {
                let response: ApiResponse<RecordImageResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, image_response);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
        }
        Status::Continue
    }
}

impl ServerHook for ImageListRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_get_method, try_get_route_param(RECORD_ID_KEY => record_id_opt), response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let record_id: i32 = match record_id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> = ApiResponse::new(
                        ApiResponseStatus::InvalidRequest,
                        ERROR_INVALID_RECORD_ID,
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_RECORD_ID_REQUIRED);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match OrderService::get_record_images(record_id).await {
            Ok(images) => {
                let response: ApiResponse<RecordImageListResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, images);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
        };
        Status::Continue
    }
}

impl ServerHook for ImageDownloadRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_get_method, try_get_route_param(ID_KEY => id_opt))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let current_user_id: i32 = match AuthService::extract_user_from_cookie(ctx) {
            Ok(user_id) => user_id,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let image_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_INVALID_IMAGE_ID);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_IMAGE_ID_REQUIRED);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match OrderService::get_image_data(image_id, current_user_id).await {
            Ok(Some(image)) => {
                let file_name: String = image
                    .try_get_original_name()
                    .clone()
                    .unwrap_or_else(|| image.get_file_name().clone());
                let content_disposition: String =
                    format!("{ATTACHMENT}; {FILENAME}=\"{}\"", file_name);
                let mime_type: String = image.get_mime_type().clone();
                let file_data: Vec<u8> = image.get_file_data().clone();
                ctx.get_mut_response()
                    .set_header(CONTENT_TYPE, &mime_type)
                    .set_header(CONTENT_DISPOSITION, &content_disposition)
                    .set_header(CONTENT_LENGTH, file_data.len().to_string())
                    .set_body(file_data);
            }
            Ok(None) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::ResourceNotFound, ERROR_IMAGE_NOT_FOUND);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
            }
        };
        Status::Continue
    }
}
