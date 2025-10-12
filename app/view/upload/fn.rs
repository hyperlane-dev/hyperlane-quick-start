use super::*;

#[route("/upload")]
#[utoipa::path(
    get,
    post,
    path = "/upload",
    responses(
        (status = 200, description = "File chunk upload frontend interface", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    response_status_code(200),
    response_body(UPLOAD_HTML),
    response_header(CONTENT_ENCODING => GZIP)
)]
pub async fn html(ctx: Context) {}

#[route("/static/{upload_dir}/{upload_file}")]
#[utoipa::path(
    get,
    post,
    path = "/static/{upload_dir}/{upload_file}",
    description = "Serve static files with optional range requests",
    responses(
        (status = 200, description = "Successfully served static file", body = String),
        (status = 206, description = "Partial content served", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    route_param(UPLOAD_DIR_KEY => dir_opt),
    route_param(UPLOAD_FILE_KEY => file_opt),
    request_header(RANGE => range_header_opt)
)]
pub async fn static_file(ctx: Context) {
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
                    match parse_range_header(&range_header, file_size) {
                        Ok(range) => Some(range),
                        Err(_) => None,
                    }
                }
                Err(_) => None,
            }
        }
        None => None,
    };
    if let Ok((partial_content, content_type)) =
        serve_static_file_with_range(&dir, &file, range_request).await
    {
        ctx.set_response_body(&partial_content.data)
            .await
            .set_response_header(CONTENT_TYPE, &content_type)
            .await
            .set_response_header(ACCEPT_RANGES, BYTES)
            .await
            .set_response_header(CACHE_CONTROL, CACHE_CONTROL_STATIC_ASSETS)
            .await
            .set_response_header(EXPIRES, EXPIRES_FAR_FUTURE)
            .await
            .set_response_header(CONTENT_LENGTH, &partial_content.total_size.to_string())
            .await;
        if has_range_request {
            ctx.set_response_status_code(HttpStatus::PartialContent.code())
                .await
                .set_response_header(CONTENT_RANGE, &partial_content.content_range)
                .await
                .set_response_header(CONTENT_LENGTH, &partial_content.content_length.to_string())
                .await;
        }
    }
}
