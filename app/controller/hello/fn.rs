use super::*;

#[post]
#[route_param(NAME_KEY => name_opt)]
#[status_code(200)]
pub async fn handle(ctx: Context) {
    let name: String = name_opt.unwrap_or_default();
    let _ = ctx.set_response_body(format!("Hello {name}")).await;
}
