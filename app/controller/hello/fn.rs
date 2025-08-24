use super::*;

#[route("/hello/{name}")]
#[prologue_hooks(
  post,
  response_status_code(200),
  route_param(NAME_KEY => name_opt),
  response_body(format!("Hello {}", name_opt.unwrap_or_default())),
)]
pub async fn handle(ctx: Context) {}
