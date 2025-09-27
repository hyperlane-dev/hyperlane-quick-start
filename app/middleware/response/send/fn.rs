use super::*;

#[response_middleware(1)]
#[epilogue_macros(http, reject(ctx.get_request_upgrade_type().await.is_ws()), send)]
pub async fn send(ctx: Context) {}
