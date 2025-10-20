use super::*;

impl ServerHook for UpgradeMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[ws]
    #[epilogue_macros(
        response_body(&vec![]),
        response_status_code(101),
        response_header(UPGRADE => WEBSOCKET),
        response_header(CONNECTION => UPGRADE),
        response_header(SEC_WEBSOCKET_ACCEPT => WebSocketFrame::generate_accept_key(ctx.try_get_request_header_back(SEC_WEBSOCKET_KEY).await.unwrap())),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}
