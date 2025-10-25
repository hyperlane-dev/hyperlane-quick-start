use super::*;

pub async fn send_body_hook(ctx: &Context) {
    let body: ResponseBody = ctx.get_response_body().await;
    if ctx.get_request().await.is_ws() {
        let frame_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
        let _: Result<(), hyperlane::ResponseError> =
            ctx.send_body_list_with_data(&frame_list).await;
    } else {
        let _: Result<(), hyperlane::ResponseError> = ctx.send_body().await;
    }
}
