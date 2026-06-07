use super::*;

/// Implementation of `IndexRoute` for `ServerHook`.
impl ServerHook for IndexRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
      methods(get, post),
      response_status_code(302),
      response_header(LOCATION => INDEX_REDIRECT_URL)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}
