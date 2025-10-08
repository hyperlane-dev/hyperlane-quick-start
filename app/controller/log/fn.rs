use super::*;

#[route("/log/info")]
#[utoipa::path(
    get,
    path = "/log/info",
    description = "View information level logs",
    responses(
        (status = 200, description = "Successfully retrieved info level logs", body = String)
    )
)]
#[prologue_macros(
    get,
    response_status_code(200),
    response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
    response_header(CONTENT_ENCODING => GZIP)
)]
pub async fn info(ctx: Context) {
    let log_content: String = read_log_file(SERVER_LOG_LEVEL[0]).await;
    ctx.set_response_body(&log_content).await;
}

#[route("/log/warn")]
#[utoipa::path(
    get,
    path = "/log/warn",
    description = "View warning level logs",
    responses(
        (status = 200, description = "Successfully retrieved warning level logs", body = String)
    )
)]
#[prologue_macros(
    get,
    response_status_code(200),
    response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
    response_header(CONTENT_ENCODING => GZIP)
)]
pub async fn warn(ctx: Context) {
    let log_content: String = read_log_file(SERVER_LOG_LEVEL[1]).await;
    ctx.set_response_body(&log_content).await;
}

#[route("/log/error")]
#[utoipa::path(
    get,
    path = "/log/error",
    description = "View error level logs",
    responses(
        (status = 200, description = "Successfully retrieved error level logs", body = String)
    )
)]
#[prologue_macros(
    get,
    response_status_code(200),
    response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
    response_header(CONTENT_ENCODING => GZIP)
)]
pub async fn error(ctx: Context) {
    let log_content: String = read_log_file(SERVER_LOG_LEVEL[2]).await;
    ctx.set_response_body(log_content).await;
}
