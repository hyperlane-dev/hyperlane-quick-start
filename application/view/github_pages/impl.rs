use super::*;

/// Handles a GitHub Pages proxy request by delegating to the service layer.
///
/// Validates that owner and repository are non-empty, performs path traversal
/// checks, then delegates to `GithubPagesService::fetch_resource` for fetching
/// and serving the resource.
///
/// For streamable content (video, audio, etc.), supports HTTP Range requests:
/// - If a `Range` header is present, returns 206 Partial Content with the
///   requested byte range and `Content-Range` header.
/// - Always includes `Accept-Ranges: bytes` for streamable content so browsers
///   know they can request partial content for seeking/buffering.
///
/// If the request path looks like a directory (no file extension, no trailing slash),
/// a 301 redirect is issued to the same path with a trailing slash appended.
/// This matches GitHub Pages behavior and ensures browsers resolve relative paths correctly.
async fn handle_github_pages_request(
    owner: String,
    repository: String,
    path: String,
    range_header_opt: Option<String>,
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
    let request_path: String = ctx.get_request().get_path().clone();
    if !request_path.ends_with('/') {
        let last_segment: &str = request_path.rsplit('/').next().unwrap_or(&request_path);
        if !last_segment.contains('.') {
            let redirect_url: String = format!("{request_path}/");
            ctx.get_mut_response()
                .set_status_code(301)
                .set_header(LOCATION, &redirect_url);
            return Status::Continue;
        }
    }
    let extension: String = FileExtension::get_extension_name(&path);
    let is_streamable: bool = STREAMABLE_EXTENSIONS.contains(&extension.as_str());
    if let Some(range_header) = &range_header_opt
        && is_streamable
    {
        match GithubPagesService::fetch_resource(&owner, &repository, &path).await {
            Ok((_, content_type)) => {
                let normalized_path: String =
                    GithubPagesService::normalize_path_static(&repository, &path);
                let local_path: String =
                    format!("{CACHE_DIR}/{owner}/{repository}/{normalized_path}");
                let file_metadata: std::fs::Metadata = match std::fs::metadata(&local_path) {
                    Ok(meta) => meta,
                    Err(_) => {
                        ctx.get_mut_response().set_status_code(502);
                        return Status::Continue;
                    }
                };
                let file_size: u64 = file_metadata.len();
                match parse_range_header(range_header, file_size) {
                    Ok((start, end)) => {
                        let content_length: u64 = end - start + 1;
                        match GithubPagesService::fetch_resource_range(
                            &owner,
                            &repository,
                            &path,
                            start,
                            end,
                        )
                        .await
                        {
                            Ok((content, _, _, _)) => {
                                let content_range: String =
                                    format!("bytes {start}-{end}/{file_size}");
                                let final_content_type: String =
                                    format_content_type(&content_type, &extension);
                                ctx.get_mut_response()
                                    .set_body(&content)
                                    .set_status_code(206)
                                    .set_header(CONTENT_TYPE, &final_content_type)
                                    .set_header(ACCEPT_RANGES, BYTES)
                                    .set_header(CONTENT_RANGE, &content_range)
                                    .set_header(CONTENT_LENGTH, content_length.to_string())
                                    .set_header(CACHE_CONTROL, NO_CACHE_NO_STORE_MUST_REVALIDATE)
                                    .set_header(PRAGMA, NO_CACHE)
                                    .set_header(EXPIRES, EXPIRES_DISABLED);
                            }
                            Err(error) => {
                                error!(
                                    "Failed to fetch resource range {owner}/{repository}/{path} {error}"
                                );
                                ctx.get_mut_response()
                                    .set_status_code(502)
                                    .set_body(format!("Failed to fetch resource: {error}"));
                            }
                        }
                    }
                    Err(_) => {
                        if let Ok((content, content_type)) =
                            GithubPagesService::fetch_resource(&owner, &repository, &path).await
                        {
                            let final_content_type: String =
                                format_content_type(&content_type, &extension);
                            ctx.get_mut_response()
                                .set_body(&content)
                                .set_status_code(200)
                                .set_header(CONTENT_TYPE, &final_content_type)
                                .set_header(ACCEPT_RANGES, BYTES)
                                .set_header(CONTENT_LENGTH, file_size.to_string())
                                .set_header(CACHE_CONTROL, NO_CACHE_NO_STORE_MUST_REVALIDATE)
                                .set_header(PRAGMA, NO_CACHE)
                                .set_header(EXPIRES, EXPIRES_DISABLED);
                        }
                    }
                }
            }
            Err(error) => {
                error!("Failed to fetch resource {owner}/{repository}/{path} {error}");
                ctx.get_mut_response()
                    .set_status_code(502)
                    .set_body(format!("Failed to fetch resource: {error}"));
            }
        }
        return Status::Continue;
    }
    match GithubPagesService::fetch_resource(&owner, &repository, &path).await {
        Ok((content, content_type)) => {
            let final_content_type: String = format_content_type(&content_type, &extension);
            let response: &mut Response = ctx
                .get_mut_response()
                .set_body(&content)
                .set_status_code(200)
                .set_header(CONTENT_TYPE, &final_content_type)
                .set_header(CACHE_CONTROL, NO_CACHE_NO_STORE_MUST_REVALIDATE)
                .set_header(PRAGMA, NO_CACHE)
                .set_header(EXPIRES, EXPIRES_DISABLED);
            if is_streamable {
                response.set_header(ACCEPT_RANGES, BYTES);
            }
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
///
/// This route matches `/github/pages/{owner}/{repository}` (without trailing slash).
/// It issues a 301 redirect to append a trailing slash, matching GitHub Pages behavior
/// where `https://{owner}.github.io/{repository}` redirects to `…/{repository}/`.
impl ServerHook for GithubPagesProxyRootRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        try_get_route_param(OWNER_KEY => owner_opt),
        try_get_route_param(REPOSITORY_KEY => repository_opt)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let owner: String = owner_opt.unwrap_or_default();
        let repository: String = repository_opt.unwrap_or_default();
        // Redirect to path with trailing slash so browsers resolve relative paths correctly.
        let redirect_url: String = format!("/github/pages/{owner}/{repository}/");
        ctx.get_mut_response()
            .set_status_code(301)
            .set_header(LOCATION, &redirect_url);
        Status::Continue
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
        try_get_route_param(OWNER_KEY => owner_opt),
        try_get_route_param(REPOSITORY_KEY => repository_opt),
        try_get_route_param(PATH_KEY => path_opt),
        try_get_request_header(RANGE => range_header_opt)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let owner: String = owner_opt.unwrap_or_default();
        let repository: String = repository_opt.unwrap_or_default();
        let path: String = path_opt.unwrap_or_default();
        handle_github_pages_request(owner, repository, path, range_header_opt, ctx).await
    }
}
