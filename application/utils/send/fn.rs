use super::*;

/// Attempts to send the response body to the client stream, handling both WebSocket and HTTP responses.
///
/// # Arguments
///
/// - `&mut Stream`: The client stream to send the response to.
/// - `&mut Context`: The request/response context containing the response data.
///
/// # Returns
///
/// - `Result<(), ResponseError>`: Ok if the response was sent successfully, or an error if sending failed.
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

/// Sends the response body to the client stream, panicking if the send fails.
///
/// # Arguments
///
/// - `&mut Stream`: The client stream to send the response to.
/// - `&mut Context`: The request/response context containing the response data.
///
/// # Panics
///
/// Panics if the underlying `try_send_body_hook` returns an error.
#[instrument_trace]
pub async fn send_body_hook(stream: &mut Stream, ctx: &mut Context) {
    try_send_body_hook(stream, ctx).await.unwrap()
}
