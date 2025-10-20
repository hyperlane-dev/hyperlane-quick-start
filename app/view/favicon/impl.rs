use super::*;

#[utoipa::path(
    get,
    path = "/favicon.ico",
    responses(
        (status = 301, description = "Redirect to favicon")
    )
)]
pub async fn ico() {}

impl ServerHook for FaviconRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_status_code(301),
        response_header(LOCATION => LOGO_IMG_URL)
    )]
    async fn handle(self, ctx: &Context) {}
}
