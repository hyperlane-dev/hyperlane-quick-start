use super::*;

#[ws]
#[route("/websocket")]
#[ws_from_stream(request)]
pub async fn handle(ctx: Context) {
    let request_body: &RequestBody = request.get_body();
    let _ = ctx.set_response_body(request_body).await.send_body().await;
}
