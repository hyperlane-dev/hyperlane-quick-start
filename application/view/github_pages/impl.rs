use super::*;

/// Handles a GitHub Pages proxy request by delegating to the service layer.
///
/// Validates that owner and repository are non-empty, performs path traversal
/// checks, then delegates to `GithubPagesService::fetch_resource` for fetching
/// and serving the resource.
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
    if path.contains("..") || path.contains('\\') {
        ctx.get_mut_response().set_status_code(403);
        return Status::Continue;
    }
    match GithubPagesService::fetch_resource(&owner, &repository, &path).await {
        Ok((content, content_type)) => {
            let extension: String = FileExtension::get_extension_name(&path);
            let response: &mut Response = ctx
                .get_mut_response()
                .set_body(&content)
                .set_status_code(200)
                .set_header(CONTENT_TYPE, content_type)
                .set_header(CACHE_CONTROL, NO_CACHE_NO_STORE_MUST_REVALIDATE)
                .set_header(PRAGMA, NO_CACHE)
                .set_header(EXPIRES, EXPIRES_DISABLED);
            if is_gzip_compressible(&extension) {
                response.set_header(CONTENT_ENCODING, GZIP);
            }
        }
        Err(error) => {
            error!("Failed to fetch resource {owner}/{repository}/{path} {error}");
            ctx.get_mut_response()
                .set_status_code(502)
                .set_body(format!("Failed to fetch resource: {error}"));
        }
    }
    Status::Continue
}

/// Implementation of `GithubPagesProxyRootRoute` for `ServerHook`.
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

/// Implementation of `GithubPagesProxyRoute` for `ServerHook`.
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
