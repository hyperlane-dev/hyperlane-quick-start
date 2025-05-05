use super::*;

pub async fn handle(ctx: Context) {
    let _ = ctx.set_response_body("Hello hyperlane").await;
}
