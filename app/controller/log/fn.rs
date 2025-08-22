use super::*;

#[utoipa::path(
    get,
    path = "/log/info",   
    responses(
        (status = 200, description = "View info level logs", body = String)
    )
)]
#[get]
#[route("/log/info")]
#[response_status_code(200)]
#[response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8))]
pub async fn info(ctx: Context) {
    let log_content: String = read_log_file("info").await;
    ctx.set_response_body(log_content).await;
}

#[utoipa::path(
    get,
    path = "/log/warn",   
    responses(
        (status = 200, description = "View warn level logs", body = String)
    )
)]
#[get]
#[route("/log/warn")]
#[response_status_code(200)]
#[response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8))]
pub async fn warn(ctx: Context) {
    let log_content: String = read_log_file("warn").await;
    ctx.set_response_body(log_content).await;
}

#[utoipa::path(
    get,
    path = "/log/error",   
    responses(
        (status = 200, description = "View error level logs", body = String)
    )
)]
#[get]
#[route("/log/error")]
#[response_status_code(200)]
#[response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8))]
pub async fn error(ctx: Context) {
    let log_content: String = read_log_file("error").await;
    ctx.set_response_body(log_content).await;
}
