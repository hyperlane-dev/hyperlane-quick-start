use super::*;

async fn handle_github_pages_request(
    owner: String,
    repository: String,
    path: String,
    ctx: &mut Context,
) -> Status {
    if owner.is_empty() || repository.is_empty() {
        ctx.get_mut_response().set_status_code(400);
        return Status::Continue;
    }
    let cache_path: String = if path.is_empty() || path == "/" {
        format!("{GITHUB_PAGES_CACHE_DIR}/{owner}/{repository}/index.html")
    } else {
        let normalized_path: String = path.trim_start_matches('/').to_string();
        format!("{GITHUB_PAGES_CACHE_DIR}/{owner}/{repository}/{normalized_path}")
    };
    if cache_path.contains("..") || cache_path.contains('\\') {
        ctx.get_mut_response().set_status_code(403);
        return Status::Continue;
    }
    match fs::read(&cache_path).await {
        Ok(content) => {
            let extension: String = FileExtension::get_extension_name(&cache_path);
            let content_type: &'static str = FileExtension::parse(&extension).get_content_type();
            ctx.get_mut_response()
                .set_body(&content)
                .set_status_code(200)
                .set_header(CONTENT_TYPE, content_type);
        }
        Err(_) => {
            let base_url: String = GITHUB_PAGES_BASE_URL_TEMPLATE
                .replace("{owner}", &owner)
                .replace("{repository}", &repository);
            let target_url: String = if path.is_empty() || path == "/" {
                base_url.clone()
            } else {
                let normalized_path: String = path.trim_start_matches('/').to_string();
                format!("{base_url}{normalized_path}")
            };
            match GithubPagesService::fetch_resource_directly(
                &owner,
                &repository,
                &path,
                &base_url,
                &target_url,
            )
            .await
            {
                Ok((content, content_type)) => {
                    ctx.get_mut_response()
                        .set_body(&content)
                        .set_status_code(200)
                        .set_header(CONTENT_TYPE, content_type);
                }
                Err(error) => {
                    error!("Failed to fetch resource directly {owner}/{repository}/{path} {error}");
                    ctx.get_mut_response()
                        .set_status_code(502)
                        .set_body(format!("Failed to fetch resource: {error}"));
                }
            }
        }
    }
    Status::Continue
}

impl ServerHook for GithubPagesProxyRootRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        try_get_route_param(GITHUB_PAGES_OWNER_KEY => owner_opt),
        try_get_route_param(GITHUB_PAGES_REPOSITORY_KEY => repository_opt)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let owner: String = owner_opt.unwrap_or_default();
        let repository: String = repository_opt.unwrap_or_default();
        handle_github_pages_request(owner, repository, String::new(), ctx).await
    }
}

impl ServerHook for GithubPagesProxyRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        try_get_route_param(GITHUB_PAGES_OWNER_KEY => owner_opt),
        try_get_route_param(GITHUB_PAGES_REPOSITORY_KEY => repository_opt),
        try_get_route_param(PATH_KEY => path_opt)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let owner: String = owner_opt.unwrap_or_default();
        let repository: String = repository_opt.unwrap_or_default();
        let path: String = path_opt.unwrap_or_default();
        handle_github_pages_request(owner, repository, path, ctx).await
    }
}
