use super::*;

#[send]
#[http]
#[flush]
pub async fn send(ctx: Context) {
    if ctx.get_request_upgrade_type().await.is_ws() {
        return;
    }
}
