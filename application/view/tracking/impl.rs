use super::*;

impl ServerHook for TrackingViewRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/tracking/index.html")
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {}
}
