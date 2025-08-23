use super::*;

#[request_middleware(2)]
#[response_header(DATE => gmt())]
#[response_header(SERVER => HYPERLANE)]
#[response_header(CONNECTION => KEEP_ALIVE)]
#[response_header(CONTENT_TYPE => TEXT_HTML)]
#[response_version(HttpVersion::HTTP1_1)]
pub async fn response_header(ctx: Context) {
    let socket_addr_string: String = ctx.get_socket_addr_or_default_string().await;
    let content_type: String = ContentType::format_content_type_with_charset(TEXT_HTML, UTF8);
    ctx.set_response_header(CONTENT_TYPE, content_type)
        .await
        .set_response_header("SocketAddr", socket_addr_string)
        .await;
}

#[request_middleware(3)]
#[response_status_code(404)]
pub async fn response_status_code(ctx: Context) {}

#[request_middleware(4)]
#[response_body(NOT_FOUND_HTML)]
pub async fn response_body(ctx: Context) {}
