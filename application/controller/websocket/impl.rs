use super::*;

impl ServerHook for WebSocketRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(ws_upgrade_type, ws_from_stream(request))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let request_body: serde_json::Result<WebSocketMessage> = request.try_get_body_json();
        if let Ok(request) = request_body {
            match WebSocketService::get_response_body(&request) {
                Ok(response) => ctx.get_mut_response().set_body(&response),
                Err(error) => ctx.get_mut_response().set_body(&error),
            };
            if try_send_body_hook(ctx).await.is_err() {
                return;
            }
        }
    }
}
