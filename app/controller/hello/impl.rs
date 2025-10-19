use super::*;

impl ServerHook for HelloRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
      methods(get, post),
      route_param(NAME_KEY => name_opt),
      response_body(format!("Hello {}", name_opt.unwrap_or_default())),
    )]
    async fn handle(self, ctx: &Context) {}
}
