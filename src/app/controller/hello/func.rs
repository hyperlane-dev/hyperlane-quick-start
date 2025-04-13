use crate::{config::server::route::NAME_KEY, *};

pub async fn handle(ctx: Context) {
    let name: String = ctx.get_route_param(NAME_KEY).await.unwrap_or_default();
    let _ = ctx.set_response_body(format!("Hello {}", name)).await;
}
