use super::*;

#[ws]
#[request_middleware(6)]
pub async fn send_body(ctx: Context) {
    ctx.set_send_body_hook(send_body_hook).await;
}
