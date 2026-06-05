use super::*;

impl ServerHook for BlogPostCreateRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        request_body_json_result(request_opt: CreateBlogPostRequest),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let request: CreateBlogPostRequest = match request_opt {
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
        match BlogService::create_post(current_user_id, request).await {
            Ok(post) => {
                let response: ApiResponse<BlogPostResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, post);
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

impl ServerHook for BlogPostUpdateRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        try_get_route_param(ID_KEY => id_opt),
        request_body_json_result(request_opt: UpdateBlogPostRequest),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let post_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_INVALID_POST_ID);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_POST_ID_REQUIRED);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let request: UpdateBlogPostRequest = match request_opt {
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
        match BlogService::update_post(post_id, current_user_id, request).await {
            Ok(post) => {
                let response: ApiResponse<BlogPostResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, post);
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

impl ServerHook for BlogPostDeleteRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        try_get_route_param(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let post_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_INVALID_POST_ID);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_POST_ID_REQUIRED);
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
        match BlogService::delete_post(post_id, current_user_id).await {
            Ok(()) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Success, SUCCESS_POST_DELETED);
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

impl ServerHook for BlogPostGetRoute {
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
        let post_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_INVALID_POST_ID);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_POST_ID_REQUIRED);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let current_user_id: Option<i32> = AuthService::extract_user_from_cookie(ctx).ok();
        match BlogService::get_post(post_id, current_user_id).await {
            Ok(Some(post)) => {
                let response: ApiResponse<BlogPostResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, post);
                ctx.get_mut_response().set_body(response.to_json_bytes())
            }
            Ok(None) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::ResourceNotFound, ERROR_POST_NOT_FOUND);
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

impl ServerHook for BlogPostListRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_get_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    #[try_get_request_query("keyword" => keyword_opt)]
    #[try_get_request_query("is_published" => is_published_opt)]
    #[try_get_request_query("page" => page_opt)]
    #[try_get_request_query("limit" => limit_opt)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let current_user_id: Option<i32> = AuthService::extract_user_from_cookie(ctx).ok();
        let mut query: BlogPostListQueryRequest = BlogPostListQueryRequest::default();
        if let Some(keyword) = keyword_opt {
            query.set_keyword(Some(keyword));
        }
        if let Some(is_published_str) = is_published_opt
            && let Ok(is_published) = is_published_str.parse::<bool>()
        {
            query.set_is_published(Some(is_published));
        }
        if let Some(page_str) = page_opt
            && let Ok(page) = page_str.parse::<i32>()
        {
            query.set_page(Some(page));
        }
        if let Some(limit_str) = limit_opt
            && let Ok(limit) = limit_str.parse::<u64>()
        {
            query.set_limit(Some(limit.min(MAX_LIST_LIMIT)));
        }
        match BlogService::list_posts(query, current_user_id).await {
            Ok(list_response) => {
                let response: ApiResponse<BlogPostListResponse> =
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

impl ServerHook for BlogPostMyListRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_get_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    #[try_get_request_query("keyword" => keyword_opt)]
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
        let mut query: BlogPostListQueryRequest = BlogPostListQueryRequest::default();
        if let Some(keyword) = keyword_opt {
            query.set_keyword(Some(keyword));
        }
        if let Some(page_str) = page_opt
            && let Ok(page) = page_str.parse::<i32>()
        {
            query.set_page(Some(page));
        }
        if let Some(limit_str) = limit_opt
            && let Ok(limit) = limit_str.parse::<u64>()
        {
            query.set_limit(Some(limit.min(MAX_LIST_LIMIT)));
        }
        match BlogService::list_my_posts(current_user_id, query).await {
            Ok(list_response) => {
                let response: ApiResponse<BlogPostListResponse> =
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

impl ServerHook for BlogPostLikeRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        try_get_route_param(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let post_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_INVALID_POST_ID);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_POST_ID_REQUIRED);
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
        match BlogService::toggle_like(post_id, current_user_id).await {
            Ok(status) => {
                let response: ApiResponse<BlogLikeStatusResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, status);
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

impl ServerHook for BlogPostFavoriteRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        try_get_route_param(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let post_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> =
                        ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_INVALID_POST_ID);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_POST_ID_REQUIRED);
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
        match BlogService::toggle_favorite(post_id, current_user_id).await {
            Ok(status) => {
                let response: ApiResponse<BlogFavoriteStatusResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, status);
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

impl ServerHook for BlogPostFavoriteListRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_get_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
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
        let mut query: BlogPostListQueryRequest = BlogPostListQueryRequest::default();
        if let Some(page_str) = page_opt
            && let Ok(page) = page_str.parse::<i32>()
        {
            query.set_page(Some(page));
        }
        if let Some(limit_str) = limit_opt
            && let Ok(limit) = limit_str.parse::<u64>()
        {
            query.set_limit(Some(limit.min(MAX_LIST_LIMIT)));
        }
        match BlogService::list_favorite_posts(current_user_id, query).await {
            Ok(list_response) => {
                let response: ApiResponse<BlogPostListResponse> =
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

impl ServerHook for BlogCommentCreateRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        request_body_json_result(request_opt: CreateBlogCommentRequest),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let request: CreateBlogCommentRequest = match request_opt {
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
        match BlogService::create_comment(current_user_id, request).await {
            Ok(comment) => {
                let response: ApiResponse<BlogCommentResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, comment);
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

impl ServerHook for BlogCommentDeleteRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        try_get_route_param(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let comment_id: i32 = match id_opt {
            Some(id_str) => match AuthService::decode_id(&id_str) {
                Ok(id) => id,
                Err(_) => {
                    let response: ApiResponse<&str> = ApiResponse::new(
                        ApiResponseStatus::InvalidRequest,
                        ERROR_INVALID_COMMENT_ID,
                    );
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_COMMENT_ID_REQUIRED);
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
        match BlogService::delete_comment(comment_id, current_user_id).await {
            Ok(()) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Success, SUCCESS_COMMENT_DELETED);
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

impl ServerHook for BlogCommentListRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_get_method, response_header(CONTENT_TYPE => APPLICATION_JSON))]
    #[instrument_trace]
    #[try_get_request_query("post_id" => post_id_opt)]
    #[try_get_request_query("page" => page_opt)]
    #[try_get_request_query("limit" => limit_opt)]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let mut query: BlogCommentListQueryRequest = BlogCommentListQueryRequest::default();
        match post_id_opt {
            Some(post_id) => query.set_post_id(post_id),
            None => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, ERROR_POST_ID_REQUIRED);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        if let Some(page_str) = page_opt
            && let Ok(page) = page_str.parse::<i32>()
        {
            query.set_page(Some(page));
        }
        if let Some(limit_str) = limit_opt
            && let Ok(limit) = limit_str.parse::<u64>()
        {
            query.set_limit(Some(limit.min(MAX_LIST_LIMIT)));
        }
        match BlogService::list_comments(query).await {
            Ok(list_response) => {
                let response: ApiResponse<BlogCommentListResponse> =
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

impl ServerHook for BlogImageUploadRoute {
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
        match BlogService::upload_image(
            current_user_id,
            file_name,
            original_name,
            mime_type,
            file_data,
        )
        .await
        {
            Ok(image_response) => {
                let response: ApiResponse<BlogImageResponse> =
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

impl ServerHook for BlogImageDownloadRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_get_method, try_get_route_param(ID_KEY => id_opt))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
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
        match BlogService::get_image_data(image_id).await {
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
