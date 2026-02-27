use super::*;

impl ServerHook for IndexRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
      methods(get, post),
      response_status_code(302),
      response_header(LOCATION => "http://120.53.248.2:66/")
  )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {}
}
