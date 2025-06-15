use super::*;

pub async fn get_name(ctx: &Context) -> String {
    ctx.get_request_query("uuid").await.unwrap_or_default()
}
