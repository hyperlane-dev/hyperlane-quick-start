use super::*;

#[instrument_trace]
pub async fn try_send_body_hook(
    stream: &mut Stream,
    ctx: &mut Context,
) -> Result<(), ResponseError> {
    let send_result: Result<(), ResponseError> = if ctx.get_request().is_ws_upgrade_type() {
        let body: &ResponseBody = ctx.get_response().get_body();
        let frame_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        stream.try_send_list(&frame_list).await
    } else {
        let body: &Vec<u8> = ctx.get_response().get_body();
        stream.try_send(body).await
    };
    if send_result.is_err() {
        stream.set_closed(true);
    }
    send_result
}

#[instrument_trace]
pub async fn send_body_hook(stream: &mut Stream, ctx: &mut Context) {
    try_send_body_hook(stream, ctx).await.unwrap()
}
