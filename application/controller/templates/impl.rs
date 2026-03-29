use super::*;

impl ServerHook for TemplatesRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_body(TEMPLATES_INDEX_HTML.replace("{{ time }}", &date()))
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {}
}
