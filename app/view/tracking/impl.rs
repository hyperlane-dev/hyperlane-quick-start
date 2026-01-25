use super::*;

impl ServerHook for TrackingViewRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_header(CONTENT_TYPE => TEXT_HTML),
        response_body(TRACKING_HTML)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {}
}
