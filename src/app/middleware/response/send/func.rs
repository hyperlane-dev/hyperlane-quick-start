use crate::*;

pub async fn send(ctx: Context) {
    let _ = ctx.send().await;
}
