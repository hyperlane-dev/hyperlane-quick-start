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
#[prologue_macros(ws, ws_from_stream(request))]
pub async fn handle(ctx: Context) {
    println_success!("WebSocket request received");
    let request_body: WebSocketMessage = request.get_body_json().unwrap();
    match get_response_body(&request_body) {
        Ok(response) => ctx.set_response_body(&response).await,
        Err(error) => ctx.set_response_body(&error).await,
    };
    ctx.try_get_send_body_hook().await.unwrap()(ctx.clone()).await;
}
