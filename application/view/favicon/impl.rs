use super::*;

impl ServerHook for FaviconRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        get_method,
        response_status_code(302),
        response_header(LOCATION => LOGO_IMG_URL)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {}
}
