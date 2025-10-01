use super::*;

#[route("/websocket")]
#[utoipa::path(
    get,
    post,
    path = "/websocket",
    responses(
        (status = 200, description = "websocket", body = String)
    )
)]
#[ws]
#[ws_from_stream(request)]
pub async fn handle(ctx: Context) {
    let request_body: &RequestBody = request.get_body();
    let _ = ctx.set_response_body(&request_body).await;
    ctx.try_get_send_body_hook().await.unwrap()(ctx.clone()).await;
}
