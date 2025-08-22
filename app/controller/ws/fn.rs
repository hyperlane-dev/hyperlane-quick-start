use super::*;

#[ws]
#[get]
#[route("/websocket")]
pub async fn handle(ctx: Context) {
    let request_body: Vec<u8> = ctx.get_request_body().await;
    let _ = ctx.set_response_body(request_body).await.send_body().await;
}
