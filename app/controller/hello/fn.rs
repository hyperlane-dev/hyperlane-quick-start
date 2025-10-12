use super::*;

#[route("/hello/{name}")]
#[prologue_macros(
  methods(get, post),
  route_param(NAME_KEY => name_opt),
  response_body(format!("Hello {}", name_opt.unwrap_or_default())),
)]
pub async fn hello_name(ctx: Context) {}
