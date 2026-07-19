use super::*;

/// Implementation of `TemplatesRoute` for `ServerHook`.
impl ServerHook for TemplatesRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        response_body(TEMPLATES_INDEX_HTML.replace("{{ time }}", &time()))
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}
