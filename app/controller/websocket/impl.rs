use super::*;

#[utoipa::path(
    get,
    path = "/api/websocket",
    responses(
        (status = 101, description = "WebSocket connection")
    )
)]
pub async fn websocket() {}

impl ServerHook for WebSocketRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(ws, ws_from_stream(request))]
    async fn handle(self, ctx: &Context) {
        println_success!("WebSocket request received");
        let request_body: WebSocketMessage = request.get_body_json().unwrap();
        match WebSocketService::get_response_body(&request_body) {
            Ok(response) => ctx.set_response_body(&response).await,
            Err(error) => ctx.set_response_body(&error).await,
        };
        send_body_hook(ctx).await;
    }
}
