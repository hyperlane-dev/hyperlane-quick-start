use super::*;

#[instrument_trace]
pub async fn try_send_body_hook(ctx: &Context) -> Result<(), ResponseError> {
    let send_result: Result<(), ResponseError> = if ctx.get_request_is_ws_upgrade_type().await {
        let body: ResponseBody = ctx.get_response_body().await;
        let frame_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&body);
        ctx.try_send_body_list_with_data(&frame_list).await
    } else {
        ctx.try_send_body().await
    };
    if send_result.is_err() {
        ctx.aborted().await.closed().await;
    }
    send_result
}
