use super::*;

impl ServerHook for WebSocketRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(ws, ws_from_stream(request))]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let request_body: serde_json::Result<WebSocketMessage> = request.try_get_body_json();
        if let Ok(request) = request_body {
            match WebSocketService::get_response_body(&request) {
                Ok(response) => ctx.set_response_body(&response).await,
                Err(error) => ctx.set_response_body(&error).await,
            };
            try_send_body_hook(ctx).await;
        }
    }
}
