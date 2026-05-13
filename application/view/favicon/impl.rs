use super::*;

impl ServerHook for FaviconRoute {
    #[instrument_trace]
    async fn new(_stream: &mut Stream, _ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        response_status_code(302),
        response_header(LOCATION => LOGO_IMG_URL)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, _ctx: &mut Context) -> Status {
        Status::Continue
    }
}
