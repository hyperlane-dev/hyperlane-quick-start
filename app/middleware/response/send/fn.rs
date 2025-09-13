use super::*;

#[response_middleware(1)]
#[epilogue_hooks(http, reject(ctx.get_request_upgrade_type().await.is_ws()), send, flush)]
pub async fn send(ctx: Context) {}
