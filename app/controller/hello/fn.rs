use super::*;

#[post]
#[route("/hello/{name}")]
#[response_status_code(200)]
#[response_body(format!("Hello {}", name_opt.unwrap_or_default()))]
#[route_param(NAME_KEY => name_opt)]
pub async fn handle(ctx: Context) {}
