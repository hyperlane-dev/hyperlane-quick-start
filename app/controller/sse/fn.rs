use super::*;

#[route("/sse")]
#[utoipa::path(
    get,
    post,
    path = "/sse",
    description = "Server-Sent Events endpoint",
    responses(
        (status = 200, description = "Successfully established SSE connection", body = String)
    )
)]
#[prologue_macros(
    methods(get, post),
    response_body(EMPTY_STR),
    response_header(CONTENT_TYPE => TEXT_EVENT_STREAM)
)]
pub async fn handle(ctx: Context) {
    let _ = ctx.send().await;
    for i in 0..10 {
        let _ = ctx
            .set_response_body(&format!("data:{}{}", i, HTTP_DOUBLE_BR))
            .await
            .send_body()
            .await;
    }
    ctx.closed().await;
}
