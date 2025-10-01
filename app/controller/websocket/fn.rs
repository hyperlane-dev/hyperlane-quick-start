use super::*;

#[route("/websocket")]
#[utoipa::path(
    get,
    post,
    path = "/websocket",
    description = "Handles incoming WebSocket connections and bidirectional communication",
    responses(
        (status = 200, description = "Successfully established WebSocket connection", body = String)
    )
)]
#[ws]
#[ws_from_stream(request)]
pub async fn handle(ctx: Context) {
    let request_body: &RequestBody = request.get_body();
    let _ = ctx.set_response_body(&request_body).await;
    ctx.try_get_send_body_hook().await.unwrap()(ctx.clone()).await;
}
