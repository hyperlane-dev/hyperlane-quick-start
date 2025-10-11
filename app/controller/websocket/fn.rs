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
    let body_string: String = String::from_utf8_lossy(request_body).into_owned();
    let response: String = get_response_body(&body_string);
    let _ = ctx.set_response_body(&response).await;
    ctx.try_get_send_body_hook().await.unwrap()(ctx.clone()).await;
}
