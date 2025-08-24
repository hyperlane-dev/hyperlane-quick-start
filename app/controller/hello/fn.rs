use super::*;

#[utoipa::path(
    get,
    path = "/hello/{name}",   
    responses(
        (status = 200, description = "Hello", body = String)
    )
)]
#[route("/hello/{name}")]
#[prologue_hooks[
  post,
  response_status_code(200),
  route_param(NAME_KEY => name_opt),
  response_body(format!("Hello {}", name_opt.unwrap_or_default())),
]]
pub async fn handle(ctx: Context) {}
