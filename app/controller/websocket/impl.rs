use super::*;

impl ServerHook for WebSocketRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("WebSocketRoute new");
        Self
    }

    #[prologue_macros(ws, ws_from_stream(request))]
    async fn handle(self, ctx: &Context) {
        trace!("WebSocketRoute handle");
        let request_body: serde_json::Result<WebSocketMessage> = request.try_get_body_json();
        if let Ok(request) = request_body {
            match WebSocketService::get_response_body(&request) {
                Ok(response) => ctx.set_response_body(&response).await,
                Err(error) => ctx.set_response_body(&error).await,
            };
            send_body_hook(ctx).await;
        }
    }
}
