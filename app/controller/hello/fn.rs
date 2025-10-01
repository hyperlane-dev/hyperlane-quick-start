use super::*;

#[route("/hello/{name}")]
#[utoipa::path(
    get,
    post,
    path = "/hello/{name}",
    description = "Greet user with name parameter",
    responses(
        (status = 200, description = "Successfully returned greeting message", body = String)
    )
)]
#[prologue_macros(
  methods(get, post),
  response_status_code(200),
  route_param(NAME_KEY => name_opt),
  response_body(format!("Hello {}", name_opt.unwrap_or_default())),
)]
pub async fn handle(ctx: Context) {}
