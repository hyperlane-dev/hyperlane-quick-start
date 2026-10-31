use super::*;

/// Implementation of `UploadViewRoute` for `ServerHook`.
impl ServerHook for UploadViewRoute {
    /// Creates a new instance.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The .
    /// - `&mut Context` - The .
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Handles one request and writes the response.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream.
    /// - `&mut Context` - The request context.
    ///
    /// # Returns
    ///
    /// - `Status` - The handle result.
    #[prologue_macros(methods(get, post))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        serve_page(ctx, UPLOAD_VIEW_PAGE_NAME)
    }
}

/// Implementation of `UploadFileRoute` for `ServerHook`.
impl ServerHook for UploadFileRoute {
    /// Creates a new instance.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The .
    /// - `&mut Context` - The .
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Handles one request and writes the response.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream.
    /// - `&mut Context` - The request context.
    ///
    /// # Returns
    ///
    /// - `Status` - The handle result.
    #[prologue_macros(
        methods(get),
        try_get_route_param(upload_dir_key() => dir_opt),
        try_get_route_param(upload_file_key() => file_opt),
    )]
    /// Implements the `handle` operation on [`UploadFileRoute`].
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Mutable reference to a `Stream` (mutated in place).
    /// - `&mut Context` - Mutable reference to a `Context` (mutated in place).
    ///
    /// # Returns
    ///
    /// - `Status` - A `Status` value.
    #[try_get_request_header(RANGE => range_header_opt)]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let dir: String = dir_opt.unwrap_or_default();
        let file: String = file_opt.unwrap_or_default();
        let has_range_request: bool = range_header_opt.is_some();
        let range_request: Option<RangeRequest> = match &range_header_opt {
            Some(range_header) => {
                let file_path: String = format!(
                    "{UPLOAD_DIR}/{}/{}",
                    Decode::execute(charsets(), &dir).unwrap_or_default(),
                    Decode::execute(charsets(), &file).unwrap_or_default()
                );
                match metadata(&file_path) {
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
            ctx.get_mut_response()
                .set_body(partial_content.get_data())
                .set_header(CONTENT_TYPE, &content_type)
                .set_header(ACCEPT_RANGES, BYTES)
                .set_header(CACHE_CONTROL, default_cache_control_static_assets())
                .set_header(EXPIRES, default_expires_far_future())
                .set_header(CONTENT_LENGTH, partial_content.get_total_size().to_string());
            if has_range_request {
                ctx.get_mut_response()
                    .set_status_code(HttpStatus::PartialContent.code())
                    .set_header(CONTENT_RANGE, partial_content.get_content_range())
                    .set_header(
                        CONTENT_LENGTH,
                        partial_content.get_content_length().to_string(),
                    );
            }
            return Status::Continue;
        }
        ctx.get_mut_response()
            .set_status_code(HttpStatus::NotFound.code())
            .set_body(HTTP_REASON_NOT_FOUND);
        Status::Continue
    }
}
