use super::*;

impl ServerHook for UploadViewRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/upload/index.html")
    )]
    async fn handle(self, ctx: &Context) {}
}

impl ServerHook for UploadFileRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        route_param_option(UPLOAD_DIR_KEY => dir_opt),
        route_param_option(UPLOAD_FILE_KEY => file_opt),
        request_header_option(RANGE => range_header_opt)
    )]
    async fn handle(self, ctx: &Context) {
        let dir: String = dir_opt.unwrap_or_default();
        let file: String = file_opt.unwrap_or_default();
        let has_range_request: bool = range_header_opt.is_some();
        let range_request: Option<RangeRequest> = match &range_header_opt {
            Some(range_header) => {
                let file_path: String = format!(
                    "{UPLOAD_DIR}/{}/{}",
                    Decode::execute(CHARSETS, &dir).unwrap_or_default(),
                    Decode::execute(CHARSETS, &file).unwrap_or_default()
                );
                match std::fs::metadata(&file_path) {
                    Ok(metadata) => {
                        let file_size: u64 = metadata.len();
                        UploadService::parse_range_header(range_header, file_size).ok()
                    }
                    Err(_) => None,
                }
            }
            None => None,
        };
        if let Ok((partial_content, content_type)) =
            UploadService::serve_static_file_with_range(&dir, &file, range_request).await
        {
            ctx.set_response_body(partial_content.get_data())
                .await
                .set_response_header(CONTENT_TYPE, &content_type)
                .await
                .set_response_header(ACCEPT_RANGES, BYTES)
                .await
                .set_response_header(CACHE_CONTROL, CACHE_CONTROL_STATIC_ASSETS)
                .await
                .set_response_header(EXPIRES, EXPIRES_FAR_FUTURE)
                .await
                .set_response_header(
                    CONTENT_LENGTH,
                    &partial_content.get_total_size().to_string(),
                )
                .await;
            if has_range_request {
                ctx.set_response_status_code(HttpStatus::PartialContent.code())
                    .await
                    .set_response_header(CONTENT_RANGE, partial_content.get_content_range())
                    .await
                    .set_response_header(
                        CONTENT_LENGTH,
                        &partial_content.get_content_length().to_string(),
                    )
                    .await;
            }
        }
    }
}
