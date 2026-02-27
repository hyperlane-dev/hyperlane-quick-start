use super::*;

#[instrument_trace]
pub async fn try_send_body_hook(ctx: &mut Context) -> Result<(), ResponseError> {
    let send_result: Result<(), ResponseError> = if ctx.get_request().is_ws_upgrade_type() {
        let body: &ResponseBody = ctx.get_response().get_body();
        let frame_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        ctx.try_send_body_list_with_data(&frame_list).await
    } else {
        ctx.try_send_body().await
    };
    if send_result.is_err() {
        ctx.set_aborted(true).set_closed(true);
    }
    send_result
}
