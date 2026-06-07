use super::*;

/// Implementation of `WebSocketRoute` for `ServerHook`.
impl ServerHook for WebSocketRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[try_get_websocket_request(request)]
    #[instrument_trace]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        match serde_json::from_slice(&request) {
            Ok(request) => {
                match WebSocketService::get_response_body(&request) {
                    Ok(response) => ctx.get_mut_response().set_body(&response),
                    Err(error) => ctx.get_mut_response().set_body(&error),
                };
            }
            Err(error) => {
                ctx.get_mut_response().set_body(error.to_string());
            }
        };
        if try_send_body_hook(stream, ctx).await.is_err() {
            return Status::Reject;
        }
    }
}
