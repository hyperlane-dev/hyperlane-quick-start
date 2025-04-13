use super::*;

pub async fn response_header(ctx: Context) {
    let socket_addr_string: String = ctx.get_socket_addr_or_default_string().await;
    let content_type: String = content_type_charset(TEXT_PLAIN, UTF8);
    ctx.set_response_header(SERVER, HYPERLANE)
        .await
        .set_response_header(CONNECTION, CONNECTION_KEEP_ALIVE)
        .await
        .set_response_header(CONTENT_TYPE, content_type)
        .await
        .set_response_header(DATE, gmt())
        .await
        .set_response_header("SocketAddr", socket_addr_string)
        .await;
}

pub async fn response_status_code(ctx: Context) {
    ctx.set_response_status_code(200).await;
}
