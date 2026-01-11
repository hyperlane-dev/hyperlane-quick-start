use super::*;

impl ServerHook for FaviconRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("FaviconRoute new");
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(301),
        response_header(LOCATION => LOGO_IMG_URL)
    )]
    async fn handle(self, ctx: &Context) {
        trace!("FaviconRoute handle");
    }
}
