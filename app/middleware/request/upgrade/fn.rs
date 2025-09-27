use super::*;

#[ws]
#[request_middleware(5)]
#[epilogue_macros(
    response_body(&vec![]),
    response_status_code(101),
    response_header(UPGRADE => WEBSOCKET),
    response_header(CONNECTION => UPGRADE),
    response_header(SEC_WEBSOCKET_ACCEPT => WebSocketFrame::generate_accept_key(&ctx.try_get_request_header_back(SEC_WEBSOCKET_KEY).await.unwrap())),
    send
)]
pub async fn upgrade(ctx: Context) {}
