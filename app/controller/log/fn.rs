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
    response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8))
)]
pub async fn info(ctx: Context) {
    let log_content: String = read_log_file("info").await;
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
    response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8))
)]
pub async fn warn(ctx: Context) {
    let log_content: String = read_log_file("warn").await;
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
    response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8))
)]
pub async fn error(ctx: Context) {
    let log_content: String = read_log_file("error").await;
    ctx.set_response_body(log_content).await;
}
