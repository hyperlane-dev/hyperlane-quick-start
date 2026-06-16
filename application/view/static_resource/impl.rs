use super::*;

/// Implementation of `StaticResourceRoute` for `ServerHook`.
///
/// Serves static files from the configured `STATIC_RESOURCES_DIR` directory.
///
/// - Text-based files (HTML, CSS, JS, etc.) include `charset=utf-8` in their
///   `Content-Type` header for proper character encoding.
/// - Streamable files (video, audio, PDF, etc.) support HTTP Range requests
///   for partial content delivery (206 status), enabling seeking/buffering
///   in browsers. When a `Range` header is present, only the requested byte
///   range is read from disk and returned.
/// - Gzip-compressible files are served with `Content-Encoding: gzip`.
impl ServerHook for StaticResourceRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        try_get_route_param(PATH_KEY => path_opt),
        try_get_request_header(RANGE => range_header_opt)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let path: String = path_opt.unwrap_or_default();
        if path.contains("..") || path.starts_with("/") || path.starts_with("\\") {
            ctx.get_mut_response().set_status_code(403);
            return Status::Continue;
        }
        let file_path: String = format!("{STATIC_RESOURCES_DIR}/{path}");
        let canonical_path: PathBuf = match fs::canonicalize(&file_path) {
            Ok(p) => p,
            Err(_) => {
                ctx.get_mut_response().set_status_code(404);
                return Status::Continue;
            }
        };
        let base_canonical: PathBuf = match fs::canonicalize(STATIC_RESOURCES_DIR) {
            Ok(p) => p,
            Err(_) => {
                ctx.get_mut_response().set_status_code(500);
                return Status::Continue;
            }
        };
        if !canonical_path.starts_with(&base_canonical) {
            ctx.get_mut_response().set_status_code(403);
            return Status::Continue;
        }
        let extension: String = FileExtension::get_extension_name(&path);
        let is_streamable: bool = STREAMABLE_EXTENSIONS.contains(&extension.as_str());
        if let Some(range_header) = &range_header_opt
            && is_streamable
        {
            let file_metadata: fs::Metadata = match fs::metadata(&file_path) {
                Ok(meta) => meta,
                Err(_) => {
                    ctx.get_mut_response().set_status_code(404);
                    return Status::Continue;
                }
            };
            let file_size: u64 = file_metadata.len();
            match parse_range_header(range_header, file_size) {
                Ok((start, end)) => {
                    let content_length: u64 = end - start + 1;
                    match read_file_range(&file_path, start, content_length).await {
                        Ok(content) => {
                            let content_range: String = format!("bytes {start}-{end}/{file_size}");
                            let raw_content_type: &'static str =
                                FileExtension::parse(&extension).get_content_type();
                            let final_content_type: String =
                                format_content_type(raw_content_type, &extension);
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
                        Err(_) => {
                            ctx.get_mut_response().set_status_code(500);
                        }
                    }
                }
                Err(_) => match fs::read(&file_path) {
                    Ok(content) => {
                        let raw_content_type: &'static str =
                            FileExtension::parse(&extension).get_content_type();
                        let final_content_type: String =
                            format_content_type(raw_content_type, &extension);
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
                    Err(_) => {
                        ctx.get_mut_response().set_status_code(404);
                    }
                },
            }
            return Status::Continue;
        }
        match fs::read(&file_path) {
            Ok(content) => {
                let raw_content_type: &'static str =
                    FileExtension::parse(&extension).get_content_type();
                let final_content_type: String = format_content_type(raw_content_type, &extension);
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
            Err(_) => {
                ctx.get_mut_response().set_status_code(404);
            }
        }
        Status::Continue
    }
}
