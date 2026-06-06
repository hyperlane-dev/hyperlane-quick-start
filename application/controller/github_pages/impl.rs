use super::*;

impl ServerHook for SyncGithubPagesRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        request_body_json_result(request_opt: SyncGithubPagesRequest),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let request: SyncGithubPagesRequest = match request_opt {
            Ok(data) => data,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error.to_string());
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let parts: Vec<&str> = request.repository.split('/').collect();
        if parts.len() != 2 {
            let response: ApiResponse<&str> = ApiResponse::new(
                ApiResponseStatus::InvalidRequest,
                ERROR_INVALID_OWNER_REPOSITORY_FORMAT,
            );
            ctx.get_mut_response().set_body(response.to_json_bytes());
            return Status::Continue;
        }
        let owner: &str = parts[0];
        let repository: &str = parts[1];
        if owner.is_empty() || repository.is_empty() {
            let response: ApiResponse<&str> = ApiResponse::new(
                ApiResponseStatus::InvalidRequest,
                ERROR_INVALID_OWNER_REPOSITORY_FORMAT,
            );
            ctx.get_mut_response().set_body(response.to_json_bytes());
            return Status::Continue;
        }
        match GithubPagesService::sync_github_pages(owner, repository).await {
            Ok(()) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Success, "Synced");
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

impl ServerHook for ListGithubPagesRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        match GithubPagesService::list_github_pages().await {
            Ok(list_response) => {
                let response: ApiResponse<GithubPagesListResponse> =
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

impl ServerHook for GetGithubPagesResourcesRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        try_get_route_param(GITHUB_PAGES_OWNER_KEY => owner_opt),
        try_get_route_param(GITHUB_PAGES_REPOSITORY_KEY => repository_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let owner: String = match owner_opt {
            Some(owner_str) => owner_str,
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    ERROR_OWNER_CANNOT_BE_EMPTY,
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let repository: String = match repository_opt {
            Some(repository_str) => repository_str,
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    ERROR_REPOSITORY_CANNOT_BE_EMPTY,
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match GithubPagesService::get_github_pages_resources(&owner, &repository).await {
            Ok(resource_response) => {
                let response: ApiResponse<GithubPagesResourceResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, resource_response);
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

impl ServerHook for DeleteGithubPagesRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_post_method,
        try_get_route_param(GITHUB_PAGES_OWNER_KEY => owner_opt),
        try_get_route_param(GITHUB_PAGES_REPOSITORY_KEY => repository_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let owner: String = match owner_opt {
            Some(owner_str) => owner_str,
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    ERROR_OWNER_CANNOT_BE_EMPTY,
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let repository: String = match repository_opt {
            Some(repository_str) => repository_str,
            None => {
                let response: ApiResponse<&str> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    ERROR_REPOSITORY_CANNOT_BE_EMPTY,
                );
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        match GithubPagesService::delete_github_pages(&owner, &repository).await {
            Ok(()) => {
                let response: ApiResponse<&str> =
                    ApiResponse::new(ApiResponseStatus::Success, "Deleted");
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
